mod http;
mod tokiort;

use {
  asahi::{
    AsahiCoordinator,
    AsahiResult,
    error,
    info
  },
  dashmap::DashMap,
  farmsim::{
    DssData,
    EndpointBuilder
  },
  futures::{
    StreamExt,
    stream::FuturesOrdered
  },
  http::fetch_dss,
  lazy_static::lazy_static
};

lazy_static! {
  static ref SERVERS_CACHE: DashMap<String, Data> = DashMap::new();
}

pub struct PollServers {
  pub db: sqlx::PgPool
}

#[derive(Debug, Clone)]
struct Server {
  friendly: String,
  internal: String,
  ip:       String,
  code:     String
}

#[derive(Debug, Clone)]
struct Data {
  dss: Option<DssData>
}

async fn fetch_data(server: Server) -> AsahiResult<Data> {
  let url = EndpointBuilder::new(&server.ip, &server.code).build();
  let dss = fetch_dss(url.stats()).await?;
  Ok(Data { dss: Some(dss) })
}

/// Stores the player's session time into database
async fn store_session_entry(
  db: &sqlx::PgPool,
  name: &str,
  uptime: i32
) -> AsahiResult<i32> {
  let kv_key = format!("player_last_uptime:{name}");

  let last_uptime: Option<i32> = sqlx::query_scalar!("SELECT value FROM kv WHERE key = $1", kv_key)
    .fetch_optional(db)
    .await?
    .and_then(|s| s.parse().ok());

  let delta = match last_uptime {
    Some(prev) => (uptime - prev).max(0),
    None => uptime
  };

  sqlx::query!(
    "INSERT INTO kv (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value",
    kv_key,
    uptime.to_string()
  )
  .execute(db)
  .await?;

  if delta == 0 {
    let result = sqlx::query_scalar!("SELECT total_played FROM players WHERE name = $1", name)
      .fetch_optional(db)
      .await?
      .unwrap_or(0);

    return Ok(result);
  }

  let result = sqlx::query_scalar!(
    "INSERT INTO players (name, total_played)
    VALUES ($1, $2) ON CONFLICT (name)
    DO UPDATE SET total_played = players.total_played + EXCLUDED.total_played
    RETURNING total_played",
    name,
    delta
  )
  .fetch_one(db)
  .await?;

  Ok(result)
}

#[asahi::async_trait]
impl AsahiCoordinator for PollServers {
  fn name(&self) -> &'static str { "Poll Servers" }

  fn interval(&self) -> u64 { 60 }

  async fn main_loop(&self) -> AsahiResult {
    let db = &self.db.clone();

    match sqlx::query_as!(Server, "SELECT friendly, internal, ip, code FROM servers")
      .fetch_all(db)
      .await
    {
      Ok(servers) => {
        let mut tasks = FuturesOrdered::new();

        for server in servers {
          tasks.push_back(async move {
            let data = fetch_data(server.clone()).await;
            (server, data)
          });
        }

        while let Some((server, result)) = tasks.next().await {
          match result {
            Ok(data) => {
              SERVERS_CACHE.insert(server.internal.clone(), data.clone());
              info!("Cache refreshed for {}", server.friendly);

              let players = match data.dss {
                Some(dss) => dss.slots.unwrap().players,
                None => Vec::with_capacity(16)
              };

              for player in players {
                if !player.is_used.unwrap_or(false) {
                  continue;
                }

                if let (Some(pname), Some(puptime)) = (player.name, player.uptime) {
                  if puptime == 0 {
                    continue;
                  }

                  match store_session_entry(db, &pname, puptime).await {
                    #[cfg(not(feature = "production"))]
                    Ok(total) => asahi::debug!("Session entry for {pname} has been inserted into database, total session is {total} minutes"),
                    #[cfg(feature = "production")]
                    Ok(_) => (),
                    Err(e) => error!("Failed to store session entry for {pname}: {e}")
                  }
                }
              }
            },
            Err(e) => error!("Failed to cache data for {}: {e}", server.friendly)
          }
        }
      },
      Err(e) => error!("Error fetching servers: {e}")
    }

    Ok(())
  }
}

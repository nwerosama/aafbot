use {
  aaf_shared::database::models::leaderboard::{
    LeaderboardConfig,
    PlayerFull,
    PlayerPartial
  },
  asahi::AsahiResult,
  farmsim::utils::fmt_uptime,
  poise::{
    CreateReply,
    serenity_prelude::{
      CreateEmbed,
      CreateEmbedFooter
    }
  },
  std::time::{
    SystemTime,
    UNIX_EPOCH
  }
};

fn slice_list(
  entries: &[PlayerPartial],
  start: usize
) -> String {
  entries
    .iter()
    .enumerate()
    .map(|(i, p)| format!("**{}.** `{}` - `{}`\n", start + i + 1, p.name, fmt_uptime(p.total_played)))
    .collect()
}

pub async fn player_card(
  player: &PlayerFull,
  database: &sqlx::Pool<sqlx::Postgres>
) -> String {
  let pos = sqlx::query_scalar!("SELECT COUNT(*) + 1 FROM players WHERE total_played > $1", player.total_played)
    .fetch_one(database)
    .await
    .expect("failed to get position count")
    .unwrap_or(1);

  let mut msg = vec![
    format!("**Position:** `#{pos}`"),
    format!("**Player name:** `{}`", player.name),
    format!("**Total played:** `{}`", fmt_uptime(player.total_played)),
  ];

  if let (Some(server), Some(date)) = (player.last_seen_server.clone(), player.last_seen_date) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let threshold = 120; // 2mins

    let last_on = if now.saturating_sub(date) < threshold {
      "playing".to_string()
    } else {
      format!("<t:{date}:R>")
    };

    msg.insert(3, format!("**Last seen:** `{server}` ({last_on})"));
  }

  msg.join("\n")
}

/// Leaderboard commands
#[poise::command(slash_command, subcommands("list", "search"))]
pub(in crate::commands) async fn leaderboard(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// View the top 50 players on the leaderboard
#[poise::command(slash_command)]
async fn list(ctx: super::PoiseContext<'_>) -> AsahiResult {
  let db = ctx.data().database.clone();
  let website = ctx.data().site_url.clone();
  let entries = sqlx::query_as!(
    PlayerPartial,
    "SELECT name, total_played FROM players ORDER BY total_played DESC LIMIT 50"
  )
  .fetch_all(&db)
  .await?;

  let date = sqlx::query_as!(LeaderboardConfig, "SELECT start_date FROM leaderboard_conf")
    .fetch_one(&db)
    .await?
    .start_date;

  let mid = entries.len().div_ceil(2);
  let (first_half, second_half) = entries.split_at(mid);

  let first = slice_list(first_half, 0);
  let second = slice_list(second_half, mid);

  ctx
    .send(
      CreateReply::default().embed(
        CreateEmbed::default()
          .color(ctx.data().embed_color)
          .title("Top 50 players on AAF servers")
          .description(
            [
              format!("Data collected since <t:{date}:D>"),
              format!("You can also find other 200 players on [our site!]({website}/leaderboard)")
            ]
            .join("\n")
          )
          .fields(vec![("\u{200b}", first, true), ("\u{200b}", second, true)])
          .footer(CreateEmbedFooter::new("Data shown is updated in background periodically"))
      )
    )
    .await
    .unwrap();

  Ok(())
}

/// Lookup the data for the given name
#[poise::command(slash_command)]
async fn search(
  ctx: super::PoiseContext<'_>,
  #[description = "In-game name to search for, e.g Nwero"] player_name: String,
  #[description = "Lowercases the name to improve search accuracy (default off)"] lowercase: Option<bool>
) -> AsahiResult {
  let database = ctx.data().database.clone();
  let name = player_name.trim();

  let lowercased = lowercase.unwrap_or(false);
  let normalized_name = if lowercased { name.to_lowercase() } else { name.to_string() };

  let entry = sqlx::query_as!(
    PlayerFull,
    "SELECT name, total_played, last_seen_server, last_seen_date FROM players WHERE name = $1",
    normalized_name
  )
  .fetch_optional(&database)
  .await?;

  if let Some(plr) = entry {
    ctx
      .send(CreateReply::default().content(player_card(&plr, &database).await))
      .await
      .unwrap();
  } else {
    let suggested = sqlx::query!("SELECT name FROM players WHERE name LIKE $1 LIMIT 10", format!("%{normalized_name}%"))
      .fetch_all(&database)
      .await?;

    let m = if suggested.is_empty() {
      "No such name found! Have they played enough in our servers?".to_string()
    } else {
      let list = suggested.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(", ");
      format!("No exact match! Possible suggested names: {list}")
    };

    ctx.send(CreateReply::default().content(m)).await.unwrap();
  }

  Ok(())
}

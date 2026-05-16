use {
  aaf_shared::database::models::leaderboard::{
    LeaderboardConfig,
    PlayerLb
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

#[derive(Debug, Clone)]
struct Player {
  /// FS player name
  name:             String,
  /// Session total in minutes
  total_played:     i32,
  /// Server name of where they were last seen on
  last_seen_server: Option<String>,
  /// Unix epoch of where they were last seen on
  last_seen_date:   Option<i64>
}

fn slice_list(
  entries: &[PlayerLb],
  start: usize
) -> String {
  entries
    .iter()
    .enumerate()
    .map(|(i, p)| format!("**{}.** `{}` - `{}`\n", start + i + 1, p.name, fmt_uptime(p.total_played)))
    .collect()
}

/// Leaderboard commands
#[poise::command(slash_command, subcommands("list", "search"))]
pub async fn leaderboard(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// View the top 50 players on the leaderboard
#[poise::command(slash_command)]
async fn list(ctx: super::PoiseContext<'_>) -> AsahiResult {
  let db = ctx.data().database.clone();
  let website = ctx.data().site_url.clone();
  let entries = sqlx::query_as!(PlayerLb, "SELECT name, total_played FROM players ORDER BY total_played DESC LIMIT 50")
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
  #[description = "In-game name to search for, e.g Nwero"] player_name: String
) -> AsahiResult {
  let database = ctx.data().database.clone();
  let name = player_name.trim();

  let entry = sqlx::query_as!(
    Player,
    "SELECT name, total_played, last_seen_server, last_seen_date FROM players WHERE name = $1",
    name
  )
  .fetch_optional(&database)
  .await?;

  if let Some(plr) = entry {
    let pos = sqlx::query_scalar!("SELECT COUNT(*) + 1 FROM players WHERE total_played > $1", plr.total_played)
      .fetch_one(&database)
      .await?
      .unwrap_or(1);

    let mut msg = vec![
      format!("**Position:** `#{pos}`"),
      format!("**Player name:** `{}`", plr.name),
      format!("**Total played:** `{}`", fmt_uptime(plr.total_played)),
    ];

    if let (Some(server), Some(date)) = (plr.last_seen_server, plr.last_seen_date) {
      let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
      let threshold = 120; // 2mins

      let last_on = if now.saturating_sub(date) < threshold {
        "playing".to_string()
      } else {
        format!("<t:{date}:R>")
      };

      msg.insert(3, format!("**Last seen:** `{server}` ({last_on})"));
    }

    ctx.send(CreateReply::default().content(msg.join("\n"))).await.unwrap();
  } else {
    let suggested = sqlx::query!("SELECT name FROM players WHERE name LIKE $1 LIMIT 10", format!("%{name}%"))
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

use {
  asahi::AsahiResult,
  farmsim::utils::fmt_uptime,
  poise::{
    CreateReply,
    serenity_prelude::{
      CreateEmbed,
      CreateEmbedFooter
    }
  }
};

#[derive(Debug, Clone)]
struct Player {
  /// FS player name
  name:         String,
  /// Session total in minutes
  total_played: i32
}

fn slice_list(
  entries: &[Player],
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
  let entries = sqlx::query_as!(Player, "SELECT name, total_played FROM players ORDER BY total_played DESC LIMIT 50")
    .fetch_all(&db)
    .await?;

  let date = sqlx::query!("SELECT value FROM kv WHERE key = 'lb_data_collection_date'")
    .fetch_one(&db)
    .await?
    .value;

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
  let name = player_name.trim().to_lowercase();

  let entry = sqlx::query_as!(Player, "SELECT name, total_played FROM players WHERE LOWER(name) = $1", name)
    .fetch_optional(&database)
    .await?;

  if let Some(plr) = entry {
    let pos = sqlx::query_scalar!("SELECT COUNT(*) + 1 FROM players WHERE total_played > $1", plr.total_played)
      .fetch_one(&database)
      .await?
      .unwrap_or(1);

    ctx
      .send(
        CreateReply::default().content(
          [
            format!("**Position:** `#{pos}`"),
            format!("**Player name:** `{}`", plr.name),
            format!("**Total played:** `{}`", fmt_uptime(plr.total_played))
          ]
          .join("\n")
        )
      )
      .await
      .unwrap();
  } else {
    let suggested = sqlx::query!("SELECT name FROM players WHERE LOWER(name) LIKE $1 LIMIT 10", format!("%{name}%"))
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

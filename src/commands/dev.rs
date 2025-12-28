use {
  asahi::AsahiResult,
  poise::{
    CreateReply,
    serenity_prelude::{
      CreateAllowedMentions,
      GenericChannelId,
      builder::CreateMessage
    }
  }
};

/// Developer commands to interact the bot with
#[poise::command(
  slash_command,
  owners_only,
  default_member_permissions = "ADMINISTRATOR",
  subcommands("echo", "leaderboard")
)]
pub async fn dev(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Echo your message as a bot
#[poise::command(slash_command)]
async fn echo(
  ctx: super::PoiseContext<'_>,
  #[description = "Message to be echoed as a bot"] message: String,
  #[description = "Channel to send this to"]
  #[channel_types("Text", "PublicThread", "PrivateThread")]
  channel: Option<GenericChannelId>
) -> AsahiResult {
  let channel = match channel {
    Some(c) => c,
    None => ctx.channel_id()
  };

  match GenericChannelId::new(channel.get())
    .send_message(
      ctx.http(),
      CreateMessage::new()
        .content(message)
        .allowed_mentions(CreateAllowedMentions::new().empty_roles().empty_users())
    )
    .await
  {
    Ok(_) => {
      ctx.send(CreateReply::new().content("Sent!").ephemeral(true)).await.unwrap();
    },
    Err(y) => {
      ctx
        .send(CreateReply::new().content(format!("Failed... `{y}`")).ephemeral(true))
        .await
        .unwrap();
      return Ok(());
    }
  }

  Ok(())
}

/// Developer commands for the leaderboard system
#[poise::command(slash_command, subcommands("timestamp", "transfer"))]
pub async fn leaderboard(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Sets the timestamp for when leaderboard was last reset
#[poise::command(slash_command)]
async fn timestamp(
  ctx: super::PoiseContext<'_>,
  #[description = "Unix epoch timestamp"] timestamp: u64
) -> AsahiResult {
  sqlx::query!(
    "INSERT INTO leaderboard_conf (start_date) VALUES ($1)
    ON CONFLICT (start_date) DO UPDATE SET start_date = EXCLUDED.start_date",
    timestamp as i64
  )
  .execute(&ctx.data().database)
  .await?;

  ctx
    .say(format!("Successfully set <t:{timestamp}:D> as the leaderboard's start date!"))
    .await
    .unwrap();

  Ok(())
}

/// Transfers the total_played value from NameA to NameB
#[poise::command(slash_command)]
async fn transfer(
  ctx: super::PoiseContext<'_>,
  #[description = "Name to transfer from"] name_a: String,
  #[description = "Name to have its combined total_played value"] name_b: String
) -> AsahiResult {
  let mut tx = ctx.data().database.begin().await?;

  let from = sqlx::query!(
    "SELECT name, total_played FROM players WHERE LOWER(name) LIKE $1",
    name_a.trim().to_lowercase()
  )
  .fetch_optional(&mut *tx)
  .await?;

  let (from_name, from_value) = match from {
    Some(r) => (r.name, r.total_played),
    None => {
      ctx
        .reply(format!("`{name_a}` is not in database, they haven't played for more than a minute!"))
        .await
        .unwrap();
      return Ok(());
    }
  };

  sqlx::query!(
    "INSERT INTO players (name, total_played) VALUES ($1, $2) ON CONFLICT (name)
    DO UPDATE SET total_played = players.total_played + EXCLUDED.total_played",
    name_b,
    from_value
  )
  .execute(&mut *tx)
  .await?;

  sqlx::query!("DELETE FROM players WHERE name = $1", from_name).execute(&mut *tx).await?;

  tx.commit().await?;

  ctx
    .reply(format!("Successfully transferred the data from **{from_name}** to **{name_b}**"))
    .await
    .unwrap();

  Ok(())
}

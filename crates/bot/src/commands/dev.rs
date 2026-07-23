use {
  crate::events::planting_info_message,
  aaf_shared::database::models::leaderboard::PlayerPartial,
  asahi::{
    AsahiResult,
    warn
  },
  farmsim::utils::fmt_uptime,
  poise::{
    ChoiceParameter,
    CreateReply,
    Modal,
    serenity_prelude::{
      CreateAllowedMentions,
      EditMessage,
      GenericChannelId,
      MessageId,
      builder::CreateMessage
    }
  },
  std::env::var
};

#[derive(ChoiceParameter)]
enum InfoChannel {
  Planting
}

impl InfoChannel {
  fn id(self) -> GenericChannelId {
    let id = match self {
      Self::Planting => var("AAF_PLANTING_INFO").expect("No 'AAF_PLANTING_INFO' key found")
    };

    GenericChannelId::new(id.parse::<u64>().unwrap())
  }
}

#[derive(Modal)]
#[name = "Echo your message as a bot"]
struct EchoModal {
  #[name = "Message to send"]
  #[placeholder = "Supports markdown only!"]
  #[max_length = 2000]
  #[paragraph]
  message: String
}

/// Developer commands to interact the bot with
#[poise::command(
  slash_command,
  owners_only,
  default_member_permissions = "ADMINISTRATOR",
  subcommands("echo", "leaderboard", "ship_info")
)]
pub async fn dev(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Echo your message as a bot
#[poise::command(slash_command)]
async fn echo(
  ctx: super::PoiseAppCtx<'_>,
  #[description = "Channel to send this to"]
  #[channel_types("Text", "PublicThread", "PrivateThread")]
  channel: Option<GenericChannelId>,
  #[description = "Supply the Message ID if you want to edit its message"] message: Option<String>
) -> AsahiResult {
  let modal = EchoModal::execute(ctx).await.expect("couldnt execute modal");

  let modal_message = match modal {
    Some(m) => m.message,
    None => {
      warn!("Modal passed empty data, not sending anything!");
      return Ok(())
    }
  };

  let channel = match channel {
    Some(c) => c,
    None => ctx.channel_id()
  };

  let channel_id = GenericChannelId::new(channel.get());

  let message_id = message.as_ref().map(|i| MessageId::new(i.parse::<u64>().expect("not a valid snowflake")));

  let allowed_mentions = CreateAllowedMentions::new().empty_roles().empty_users();

  let reply = |content: String| async move {
    ctx.send(CreateReply::new().content(content).ephemeral(true)).await.unwrap();
  };

  if let Some(message_id) = message_id {
    match channel_id
      .edit_message(
        ctx.http(),
        message_id,
        EditMessage::new().content(&modal_message).allowed_mentions(allowed_mentions.clone())
      )
      .await
    {
      Ok(_) => reply("Edited!".to_owned()).await,
      Err(y) => {
        reply(format!("Failed... `{y}`")).await;
        return Ok(());
      }
    }
  } else {
    match channel_id
      .send_message(
        ctx.http(),
        CreateMessage::new().content(&modal_message).allowed_mentions(allowed_mentions)
      )
      .await
    {
      Ok(_) => reply("Sent!".to_owned()).await,
      Err(y) => {
        reply(format!("Failed... `{y}`")).await;
        return Ok(());
      }
    }
  }

  Ok(())
}

/// Ship an info message to specified channel, f.e. planting info
#[poise::command(slash_command)]
async fn ship_info(
  ctx: super::PoiseContext<'_>,
  #[description = "Info channel to ship this to"] channel: InfoChannel,
  #[description = "Edit existing message to update info with"] message_id: Option<String>
) -> AsahiResult {
  ctx.defer_ephemeral().await.unwrap();

  let channel_id = channel.id();

  let message_id = message_id.map(|i| MessageId::new(i.parse::<u64>().expect("not a valid snowflake")));

  match channel {
    InfoChannel::Planting => planting_info_message(ctx.serenity_context(), channel_id, message_id).await
  }

  if message_id.is_some() {
    ctx.reply(format!("Updated the message in <#{}>", channel_id.get())).await.unwrap();
  } else {
    ctx.reply(format!("Shipped it to <#{}>", channel_id.get())).await.unwrap();
  }

  Ok(())
}

/// Developer commands for the leaderboard system
#[poise::command(slash_command, subcommands("timestamp", "transfer", "destroy"))]
pub async fn leaderboard(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Sets the timestamp for when leaderboard was last reset
#[poise::command(slash_command)]
async fn timestamp(
  ctx: super::PoiseContext<'_>,
  #[description = "Unix epoch timestamp"] timestamp: u64
) -> AsahiResult {
  sqlx::query_as!(
    LeaderboardConfig,
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

  let from = sqlx::query_as!(PlayerPartial, "SELECT name, total_played FROM players WHERE name LIKE $1", name_a.trim())
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

  let to_before = sqlx::query!("SELECT name, total_played FROM players WHERE name LIKE $1", name_b.trim())
    .fetch_optional(&mut *tx)
    .await?;

  let to_before_val = to_before.as_ref().map(|r| r.total_played).unwrap_or(0);
  let combined_val = from_value + to_before_val;

  sqlx::query_as!(
    PlayerPartial,
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
    .reply(
      [
        "Data transferred successfully!",
        &format!("- **{from_name}:** `{}`", fmt_uptime(from_value)),
        &format!("- **{name_b}:** `{}`", fmt_uptime(to_before_val)),
        &format!("Total combined: `{}`", fmt_uptime(combined_val))
      ]
      .join("\n")
    )
    .await
    .unwrap();

  Ok(())
}

/// Destroy the player's leaderboard data
#[poise::command(slash_command)]
async fn destroy(
  ctx: super::PoiseContext<'_>,
  #[description = "Player name to destroy their data"] player: String
) -> AsahiResult {
  let mut tx = ctx.data().database.begin().await?;

  let sess = sqlx::query!("DELETE FROM sessions WHERE name = $1", player).execute(&mut *tx).await?;
  let plr = sqlx::query!("DELETE FROM players WHERE name = $1", player).execute(&mut *tx).await?;

  tx.commit().await.expect("error committing transaction");

  let unaffected = sess.rows_affected() < 1 && plr.rows_affected() < 1;

  if unaffected {
    ctx.reply("Can't destroy data if it's not present!").await.unwrap();
  } else {
    ctx.reply("Data destroyed!").await.unwrap();
  }

  Ok(())
}

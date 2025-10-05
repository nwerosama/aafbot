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
#[poise::command(slash_command, owners_only, default_member_permissions = "ADMINISTRATOR", subcommands("echo"))]
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

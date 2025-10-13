use {
  crate::BotData,
  asahi::{
    AsahiError,
    error
  },
  poise::{
    FrameworkError,
    serenity_prelude::Mentionable
  }
};

pub async fn fw_errors(error: FrameworkError<'_, BotData, AsahiError>) {
  match error {
    FrameworkError::Command { error, ctx, .. } => {
      if (ctx
        .reply(format!(
          "Encountered an error during command execution, please notify {}!",
          ctx.data().notify_dev.mention()
        ))
        .await)
        .is_err()
      {
        error!("PoiseCommandError({}): {error}", ctx.command().qualified_name)
      }
      error!("PoiseCommandErrorDebug({}): {error:?}", ctx.command().qualified_name);
    },
    FrameworkError::CommandPanic { payload, ctx, .. } => {
      if (ctx
        .reply(format!("The command panicked, please notify {}!", ctx.data().notify_dev.mention()))
        .await)
        .is_err()
      {
        error!("PoiseCommandPanic({}): {payload:#?}", ctx.command().qualified_name);
      }
    },
    FrameworkError::ArgumentParse { error, input, ctx, .. } => {
      let input = input.unwrap_or_else(|| "<blank>".to_string());
      if (ctx.reply(format!("Wrong command argument! Used `{input}`, error: `{error}`")))
        .await
        .is_err()
      {
        error!("PoiseArgumentParseError({input}): {error:?}")
      }
      error!("PoiseArgumentParseError({input}): {error:?}")
    },
    FrameworkError::NotAnOwner { ctx, .. } => {
      error!(
        "PoiseNotAnOwner: {} tried to execute a developer-level command ({})",
        ctx.author().name,
        ctx.command().qualified_name
      );
      ctx
        .reply("This command is only usable by the developers and you're not one of them!")
        .await
        .expect("Error sending message");
    },
    FrameworkError::UnknownCommand { msg, .. } => error!("PoiseUnknownCommand: {} tried to run a command that doesn't exist!", msg.author.name),
    other => error!("PoiseOtherError: {other:?}")
  }
}

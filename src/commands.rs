mod dev;
mod settings;
mod stats;

use {
  dev::dev,
  settings::settings,
  stats::stats
};

use {
  crate::BotData,
  asahi::{
    AsahiError,
    AsahiResult
  }
};

pub type PoiseCmdData = Vec<poise::Command<BotData, AsahiError>>;
pub type PoiseContext<'a> = poise::Context<'a, BotData, AsahiError>;

pub fn collect() -> PoiseCmdData { vec![deploy(), dev(), settings(), stats()] }

/// Deploy the commands
#[poise::command(prefix_command, owners_only)]
pub async fn deploy(ctx: PoiseContext<'_>) -> AsahiResult {
  if poise::builtins::register_in_guild(ctx.http(), &collect(), ctx.data().main_guild)
    .await
    .is_ok()
  {
    ctx.reply("Deployed the commands successfully!").await.unwrap();
  }

  Ok(())
}

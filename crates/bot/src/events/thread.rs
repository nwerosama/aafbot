mod appy;

use {
  asahi::AsahiResult,
  poise::serenity_prelude::{
    Context,
    GuildThread
  }
};

pub async fn on_create(
  ctx: &Context,
  thread: &GuildThread,
  newly_created: &Option<bool>
) -> AsahiResult {
  if newly_created == &Some(false) {
    return Ok(())
  }

  appy::applications(ctx, thread).await.unwrap();

  Ok(())
}

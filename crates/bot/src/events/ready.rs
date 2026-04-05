use {
  asahi::{
    AsahiResult,
    info
  },
  poise::serenity_prelude::{
    Context,
    Ready
  },
  std::sync::atomic::{
    AtomicBool,
    Ordering
  }
};

static READY_ONCE: AtomicBool = AtomicBool::new(false);

async fn ready_once(
  #[cfg(not(feature = "production"))] ctx: &Context,
  #[cfg(feature = "production")] _: &Context,
  ready: &Ready
) -> AsahiResult {
  #[cfg(not(feature = "production"))]
  {
    asahi::warn!("You are running in development mode!");
    let gw = ctx.http.get_bot_gateway().await.expect("API not available");
    let sess = gw.session_start_limit;
    info!("Gateway session limit: {}/{}", sess.remaining, sess.total)
  }

  info!("Connected to Discord as {}", ready.user.name);

  Ok(())
}

pub async fn on_ready(
  ctx: &Context,
  ready: &Ready
) -> AsahiResult {
  if !READY_ONCE.swap(true, Ordering::Relaxed) {
    ready_once(ctx, ready).await.expect("failed to call on_ready method");
  }
  Ok(())
}

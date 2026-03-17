#[derive(Debug)]
pub struct BotData {
  pub embed_color: u32,
  pub main_guild:  poise::serenity_prelude::GuildId,
  /// Lead developer
  pub notify_dev:  poise::serenity_prelude::UserId,
  pub database:    sqlx::PgPool,
  /// Base url of the community site
  pub site_url:    String,
  pub emojis:      Emojis
}

#[derive(Debug, Clone, Copy)]
pub struct Emojis {
  pub fs22: u64,
  pub fs25: u64
}

#[derive(Debug)]
pub struct BotData {
  pub embed_color: u32,
  pub main_guild:  poise::serenity_prelude::GuildId,
  /// Lead developer
  pub notify_dev:  poise::serenity_prelude::UserId,
  pub database:    sqlx::PgPool,
  /// Base url of the community site
  pub site_url:    String
}

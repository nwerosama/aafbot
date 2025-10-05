use {
  asahi::AsahiResult,
  poise::serenity_prelude::{
    AutocompleteChoice,
    // ChannelId,
    CreateAutocompleteResponse
  }
};

async fn ar_ac_kw<'a>(
  ctx: super::PoiseContext<'_>,
  partial: &'a str
) -> CreateAutocompleteResponse<'a> {
  let guild_id = ctx.guild_id().map(|g| g.get() as i64);
  if guild_id.is_none() {
    return CreateAutocompleteResponse::new().add_choice("Nothing to populate the list, add some first!");
  }

  let guild_id = guild_id.expect("expected guild id to be present");

  let rows = sqlx::query!("SELECT keywords FROM autoresponders WHERE guild_id = $1", guild_id)
    .fetch_all(&ctx.data().database)
    .await
    .unwrap_or_default();

  let mut choices = Vec::new();

  for row in rows {
    for kw in row.keywords {
      if kw.starts_with(partial) {
        choices.push(AutocompleteChoice::new(kw.clone(), kw))
      }
    }
  }

  CreateAutocompleteResponse::new().set_choices(choices)
}

/// Manage settings for namespaces in the bot
#[poise::command(slash_command, default_member_permissions = "MANAGE_GUILD", subcommands("autoresponders"))]
pub async fn settings(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Settings for autoresponders namespace
#[poise::command(slash_command, subcommands("ar_add", "ar_del"))]
async fn autoresponders(_: super::PoiseContext<'_>) -> AsahiResult { Ok(()) }

/// Add/update the autoresponder
#[poise::command(slash_command, rename = "add")]
async fn ar_add(
  ctx: super::PoiseContext<'_>,
  #[description = "Channel to respond in"]
  #[channel_types("Text")]
  // channel: ChannelId,
  // workaround, discord mustve broken something on their end as Poise is failing to parse it...
  channel: String,
  #[description = "Keywords (separate each word by comma if wanting multiple for same response)"] keywords: String,
  #[description = "Response text to reply back to user with"] response: String
) -> AsahiResult {
  let guild_id = ctx.guild_id().expect("should be guild data present").get() as i64;
  // let channel_id = channel.get() as i64;
  let channel_id = channel.parse::<i64>().expect("expected valid channel id as integer");

  let kw: Vec<String> = keywords.split(',').map(|s| s.trim().to_string()).collect();
  let first_kw = kw.first().cloned().unwrap_or_else(|| "<blank>".to_string());

  let exists = sqlx::query_scalar!(
    "SELECT EXISTS (
      SELECT 1 FROM autoresponders
      WHERE guild_id = $1 AND channel_id = $2 AND keywords = $3
    )",
    guild_id,
    channel_id,
    &kw
  )
  .fetch_one(&ctx.data().database)
  .await?;

  sqlx::query!(
    "INSERT INTO autoresponders (guild_id, channel_id, keywords, response)
      VALUES ($1, $2, $3, $4)
        ON CONFLICT (guild_id, channel_id, keywords)
        DO UPDATE SET response = EXCLUDED.response
    ",
    guild_id,
    channel_id,
    &kw,
    response
  )
  .execute(&ctx.data().database)
  .await?;

  let fmt_m = format!("Autoresponder for `{first_kw}`");
  let msg = if exists.unwrap_or(false) {
    format!("{fmt_m} updated!")
  } else {
    format!("{fmt_m} created!")
  };

  ctx.reply(msg).await.unwrap();

  Ok(())
}

/// Delete the existing autoresponder
#[poise::command(slash_command, rename = "delete")]
async fn ar_del(
  ctx: super::PoiseContext<'_>,
  #[description = "Autoresponder keyword to delete"]
  #[autocomplete = "ar_ac_kw"]
  keywords: String
) -> AsahiResult {
  let guild_id = ctx.guild_id().expect("should be guild data present").get() as i64;
  let kw: Vec<String> = keywords.split(',').map(|s| s.trim().to_string()).collect();
  let first_kw = kw.first().cloned().unwrap_or_else(|| "<blank>".to_string());

  let res = sqlx::query!("DELETE FROM autoresponders WHERE guild_id = $1 AND keywords = $2", guild_id, &kw)
    .execute(&ctx.data().database)
    .await?;

  let msg = if res.rows_affected() > 0 {
    format!("Autoresponder for `{first_kw}` deleted!")
  } else {
    format!("No autoresponder found for `{first_kw}` in this channel!")
  };

  ctx.reply(msg).await.unwrap();

  Ok(())
}

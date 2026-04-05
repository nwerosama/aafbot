use {
  crate::data::BotData,
  asahi::AsahiResult,
  poise::serenity_prelude::{
    Context,
    Message
  }
};

async fn autoresponder(
  ctx: &Context,
  message: &Message
) -> AsahiResult {
  let content = message.content.to_lowercase();

  if let Some(row) = sqlx::query!(
    "SELECT response from autoresponders WHERE channel_id = $1 AND $2 = ANY(keywords) LIMIT 1",
    message.channel_id.get() as i64,
    &content
  )
  .fetch_optional(&ctx.data::<BotData>().database)
  .await
  .unwrap()
  {
    message.reply(&ctx.http, &row.response).await.unwrap();
  }

  Ok(())
}

pub async fn on_message(
  ctx: &Context,
  new_message: &Message
) -> AsahiResult {
  if new_message.author.bot() {
    return Ok(());
  }

  autoresponder(ctx, new_message).await.unwrap();

  Ok(())
}

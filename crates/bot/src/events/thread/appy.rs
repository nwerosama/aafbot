use {
  crate::{
    commands::leaderboard::player_card,
    data::BotData
  },
  aaf_shared::database::models::leaderboard::PlayerFull,
  asahi::{
    AsahiResult,
    error,
    warn
  },
  poise::serenity_prelude::{
    Context,
    CreateMessage,
    Embed,
    GenericChannelId,
    GetMessages,
    GuildThread,
    UserId,
    small_fixed_array::FixedArray
  },
  std::env::var
};

static APPY_ID: UserId = UserId::new(853327905357561948);

fn app_channels() -> Vec<GenericChannelId> {
  let staff_apps_raw = var("AAF_STAFF_APPS")
    .expect("No 'AAF_STAFF_APPS' key found")
    .parse::<u64>()
    .expect("parsing failed");
  let plant_apps_raw = var("AAF_PLANTING_APPS")
    .expect("No 'AAF_PLANTING_APPS' key found")
    .parse::<u64>()
    .expect("parsing failed");
  vec![GenericChannelId::new(staff_apps_raw), GenericChannelId::new(plant_apps_raw)]
}

fn sanitize_value(v: &str) -> String { v.trim().trim_matches('`').trim().to_string() }

fn extract_ign(embeds: &FixedArray<Embed, u32>) -> Option<String> {
  for embed in embeds {
    let desc = embed.description.as_ref()?;
    let mut lines = desc.lines().map(str::trim).filter(|l| !l.is_empty());

    while let Some(line) = lines.next() {
      if line.to_lowercase().contains("gamertag")
        && let Some(ans) = lines.next()
      {
        let ign = sanitize_value(ans);
        if !ign.is_empty() {
          return Some(ign)
        }
      }
    }
  }

  None
}

pub async fn applications(
  ctx: &Context,
  thread: &GuildThread
) -> AsahiResult {
  let pid = thread.parent_id;
  let thread_name = thread.base.name.clone();
  let database = ctx.data::<BotData>().database.clone();

  if !app_channels().contains(&pid.widen()) || thread.owner_id != APPY_ID {
    return Ok(())
  }

  let messages = match pid.widen().messages(&ctx.http, GetMessages::new().limit(3)).await {
    Ok(m) => m,
    Err(e) => {
      error!("failed to fetch messages in parent channel for our app thread ({thread_name}): {e:?}");
      return Ok(())
    }
  };

  let Some(message) = messages
    .iter()
    .find(|m| m.author.id == APPY_ID && m.thread.as_ref().is_some_and(|t| t.id == thread.id))
  else {
    warn!("couldnt find the starter message for our app thread ({thread_name})");
    return Ok(())
  };

  let Some(ign) = extract_ign(&message.embeds) else {
    warn!("could not extract IGN from embed!");
    return Ok(())
  };

  if ign.is_empty() {
    warn!("extracted ign from embed but it is empty!");
    return Ok(())
  }

  let Some(player) = sqlx::query_as!(PlayerFull, "SELECT * FROM players WHERE LOWER(name) = LOWER($1)", ign)
    .fetch_optional(&database)
    .await?
  else {
    warn!(
      "Player search yielded no results for {ign} - application {}:{} (ChID:MsgID)",
      message.channel_id, message.id
    );
    return Ok(())
  };

  let recent = thread
    .id
    .widen()
    .messages(&ctx.http, GetMessages::new().limit(3))
    .await
    .unwrap_or_default();
  if recent.iter().any(|m| m.author.id == ctx.cache.current_user().id) {
    return Ok(())
  }

  thread
    .send_message(&ctx.http, CreateMessage::default().content(player_card(&player, &database).await))
    .await
    .expect("failed to send message in app thread");

  Ok(())
}

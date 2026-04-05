mod interaction;
mod message;
mod ready;

// todo; move planting_info_message func to suitable place
// in the future if we add more info channels to this bot
pub use interaction::planting_info_message;

use {
  asahi::Probe,
  extract_map::ExtractMap,
  poise::serenity_prelude::{
    ConnectionStage,
    Context,
    Event,
    EventHandler,
    FullEvent,
    async_trait
  },
  std::sync::Arc
};

pub struct DiscordEvents {
  pub probe: Arc<Probe>
}

#[async_trait]
impl EventHandler for DiscordEvents {
  async fn dispatch(
    &self,
    ctx: &Context,
    event: &FullEvent
  ) {
    match event {
      FullEvent::Ready { data_about_bot, .. } => ready::on_ready(ctx, data_about_bot).await.unwrap(),
      FullEvent::ShardStageUpdate { event, .. } => match event.new {
        ConnectionStage::Connected => self.probe.update_status(true).await,
        ConnectionStage::Resuming => self.probe.update_status(true).await,
        ConnectionStage::Disconnected => self.probe.update_status(false).await,
        _ => ()
      },
      FullEvent::InteractionCreate { interaction, .. } => interaction::on_interaction(ctx, interaction).await.unwrap(),
      FullEvent::Message { new_message, .. } => message::on_message(ctx, new_message).await.unwrap(),
      _ => ()
    }
  }

  fn filter_event(
    &self,
    _ctx: &Context,
    mut event: Box<Event>
  ) -> Option<Box<Event>> {
    // filter unimportant events out as i do not do anything with it to reduce
    // memory footprint as much as possible.
    match &mut *event {
      Event::MessageUpdate(_) => return None,
      Event::GuildCreate(evt) => {
        evt.guild.emojis = ExtractMap::new();
        evt.guild.stickers = ExtractMap::new();
      },
      Event::GuildUpdate(evt) => {
        evt.guild.emojis = ExtractMap::new();
        evt.guild.stickers = ExtractMap::new();
      },
      Event::GuildEmojisUpdate(evt) => evt.emojis = ExtractMap::new(),
      Event::GuildStickersUpdate(evt) => evt.stickers = ExtractMap::new(),
      _ => ()
    }

    Some(event)
  }
}

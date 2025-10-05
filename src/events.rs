mod message;
mod ready;

use {
  asahi::Probe,
  poise::serenity_prelude::{
    ConnectionStage,
    Context,
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
      FullEvent::Message { new_message, .. } => message::on_message(ctx, new_message).await.unwrap(),
      _ => ()
    }
  }
}

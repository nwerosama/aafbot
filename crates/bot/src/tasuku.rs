mod birthday;
pub mod poll_servers;

use {
  crate::BotData,
  poise::serenity_prelude::Http,
  std::sync::Arc
};

pub struct Tasuku {
  data: Arc<BotData>,
  http: Arc<Http>
}

impl Tasuku {
  pub fn new(
    data: Arc<BotData>,
    http: Arc<Http>
  ) -> Self {
    Self { data, http }
  }

  pub fn init(&self) {
    asahi::spawn(poll_servers::PollServers {
      db: self.data.database.clone(),
      #[cfg(feature = "production")]
      first_run: std::sync::atomic::AtomicBool::new(true)
    });

    asahi::spawn(birthday::Birthday {
      db:     self.data.database.clone(),
      http:   self.http.clone(),
      accent: self.data.embed_color
    })
  }
}

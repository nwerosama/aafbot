pub mod poll_servers;

use {
  asahi::spawn,
  tokio::task::JoinHandle
};

pub async fn init(db: sqlx::PgPool) -> JoinHandle<()> { spawn(poll_servers::PollServers { db }) }

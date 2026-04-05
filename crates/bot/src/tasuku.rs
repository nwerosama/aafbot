pub mod poll_servers;

pub fn init(db: sqlx::PgPool) { asahi::spawn(poll_servers::PollServers { db }) }

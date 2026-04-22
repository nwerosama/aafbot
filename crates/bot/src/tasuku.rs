pub mod poll_servers;

pub fn init(db: sqlx::PgPool) {
  asahi::spawn(poll_servers::PollServers {
    db,
    #[cfg(feature = "production")]
    first_run: std::sync::atomic::AtomicBool::new(true)
  })
}

pub mod models;

use {
  crate::load_env,
  asahi::{
    error,
    info,
    utils::database::{
      AsahiDatabaseConfig,
      connect
    }
  },
  sqlx::{
    Pool,
    Postgres
  }
};

pub struct Database(String, String);

impl AsahiDatabaseConfig for Database {
  fn uri(&self) -> &str { &self.0 }

  fn app_name(&self) -> &str { &self.1 }

  fn max_connections(&self) -> u32 { 10 }
}

pub async fn init(name: &str) -> Pool<Postgres> {
  let uri = load_env("DATABASE_URL");
  let conf = Database(uri, name.to_owned());

  let pool = connect(&conf).await;

  match pool {
    Ok(p) => {
      info!("Connected to database at {}", p.connect_options().get_host());
      p
    },
    Err(e) => {
      error!("Unable to connect to database: {e}");
      std::process::exit(1)
    }
  }
}

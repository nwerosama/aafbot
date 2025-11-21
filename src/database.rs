use {
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
  },
  std::env::var
};

pub struct Database(String);

impl AsahiDatabaseConfig for Database {
  fn uri(&self) -> &str { &self.0 }

  fn app_name(&self) -> &str { "AAFBot" }

  fn max_connections(&self) -> u32 { 10 }
}

pub async fn init() -> Pool<Postgres> {
  let conf = Database(var("DATABASE_URL").expect("No 'DATABASE_URL' found!"));

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

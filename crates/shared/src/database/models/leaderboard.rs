use serde::{
  Deserialize,
  Serialize
};

/// Returns only the data for `name` and `total_played`
///
/// Use [`PlayerFull`] for every column in database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerPartial {
  /// FS player name
  pub name:         String,
  /// Session total in minutes
  pub total_played: i32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerFull {
  /// FS player name
  pub name:             String,
  /// Session total in minutes
  pub total_played:     i32,
  /// Server name of where they were last seen on
  pub last_seen_server: Option<String>,
  /// Unix epoch of where they were last seen on
  pub last_seen_date:   Option<i64>
}

#[derive(Debug, Clone)]
pub struct LeaderboardConfig {
  /// Data collection start date in Unix epoch
  pub start_date: i64
}

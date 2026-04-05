#[derive(Debug, Clone)]
pub struct PlayerLb {
  /// FS player name
  pub name:         String,
  /// Session total in minutes
  pub total_played: i32
}

#[derive(Debug, Clone)]
pub struct LeaderboardConfig {
  /// Data collection start date in Unix epoch
  pub start_date: i64
}

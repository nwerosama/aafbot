#[derive(Debug, Clone)]
pub struct Server {
  /// Friendly server name, e.g. `Silage 25`
  pub friendly: String,
  /// Internal server name, e.g. `silage25`
  ///
  /// *For use with application internals only*
  pub internal: String,
  /// IP address or FQDN
  pub ip:       String,
  /// MD5 code/API code
  ///
  /// This is found in settings page in Dediserver webinterface
  pub code:     String
}

#[derive(Debug, Clone)]
pub struct ServerWithGameVersion {
  /// Friendly server name, e.g. `Silage 25`
  pub friendly:     String,
  /// Internal server name, e.g. `silage25`
  ///
  /// *For use with application internals only*
  pub internal:     String,
  /// IP address or FQDN
  pub ip:           String,
  /// MD5 code/API code
  ///
  /// This is found in settings page in Dediserver webinterface
  pub code:         String,
  /// Game version, e.g. `25`
  pub game_version: Option<i16>
}

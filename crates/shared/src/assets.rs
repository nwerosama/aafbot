/// Manifest file is automatically generated upon request call after the assets
/// server walks the directory of the given path
#[derive(serde::Deserialize)]
pub struct Manifest {
  /// Retrieved from `timestamp.txt` in the directory
  pub timestamp: i64,
  /// List of visible media in the directory
  pub media:     Vec<String>
}

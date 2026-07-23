pub mod assets;
#[cfg(not(target_arch = "wasm32"))]
pub mod database;

/// Injects envvar from elsewhere into application
pub fn load_env(s: &str) -> String { std::env::var(s).unwrap_or_else(|_| panic!("No '{s}' key found")) }

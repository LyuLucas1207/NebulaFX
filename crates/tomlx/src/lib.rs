mod error;
mod loader;

pub use error::{TomlConfigError, Result};
pub use loader::{load_config_from_path, load_config_from_str};

pub fn load_config<T>(path: impl AsRef<std::path::Path>) -> Result<T> where T: serde::de::DeserializeOwned {
    load_config_from_path(path)
}

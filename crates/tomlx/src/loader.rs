use crate::error::{Result, TomlConfigError};
use std::path::Path;

pub fn load_config_from_path<T>(path: impl AsRef<Path>) -> Result<T> where T: serde::de::DeserializeOwned {
    let path = path.as_ref();
    
    if !path.exists() {
        return Err(TomlConfigError::NotFound(
            path.display().to_string(),
        ));
    }

    let content = std::fs::read_to_string(path).map_err(|e| TomlConfigError::Io(e))?;

    load_config_from_str(&content)
}

pub fn load_config_from_str<T>(content: &str) -> Result<T> where T: serde::de::DeserializeOwned {
    let config: T = toml::from_str(content).map_err(|e| TomlConfigError::Parse(e))?;

    Ok(config)
}

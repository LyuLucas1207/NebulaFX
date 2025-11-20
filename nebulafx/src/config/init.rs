use crate::config::Config;
use nebulafx_tomlx::{load_config_from_path, Result};
use std::env;

pub fn load_config() -> Result<Config> {
    load_config_from_path(if env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string()) == "dev" {
        "config.dev.toml"
    } else {
        "config.toml"
    })
}


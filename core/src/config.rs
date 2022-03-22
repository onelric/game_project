use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WindowConfig {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Config {
    pub window: WindowConfig,
}

pub fn get_config() -> Config {
    toml::from_slice(&std::fs::read(PathBuf::from("./config.toml")).unwrap()).unwrap()
}

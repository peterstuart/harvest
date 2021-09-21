use crate::{Auth, Result};
use anyhow::anyhow;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub auth: Auth,
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let path = Self::path()?;
        let contents = fs::read_to_string(&path)
            .map_err(|_| anyhow!("Expected config file at {}", path.to_str().unwrap()))?;
        Ok(serde_yaml::from_str(&contents)?)
    }

    fn path() -> Result<PathBuf> {
        let mut path = dirs::config_dir().ok_or_else(|| anyhow!(""))?;
        path.push("harvest");
        path.push("config.yml");

        Ok(path)
    }
}

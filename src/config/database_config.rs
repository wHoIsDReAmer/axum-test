use std::path::PathBuf;

use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize)]
pub(crate) struct DatabaseConfig {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
}

#[allow(unused)]
impl DatabaseConfig {
    pub(crate) fn new(host: String, port: u16, user: String, password: String, database: String) -> Self {
        Self { host, port, user, password, database }
    }
}

impl TryFrom<PathBuf> for DatabaseConfig {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }
}
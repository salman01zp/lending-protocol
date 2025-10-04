// Configuration module for the Miden lending protocol client

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_endpoint: String,
    pub lending_pool_account_id: Option<String>,
    pub price_oracle_account_id: Option<String>,
    pub user_account_id: Option<String>,
    pub storage_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_endpoint: "http://localhost:57291".to_string(),
            lending_pool_account_id: None,
            price_oracle_account_id: None,
            user_account_id: None,
            storage_path: PathBuf::from(".miden-lending"),
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // TODO: Load from file if exists, otherwise return default
        Ok(Self::default())
    }

    pub fn save(&self) -> anyhow::Result<()> {
        // TODO: Save configuration to file
        Ok(())
    }
}

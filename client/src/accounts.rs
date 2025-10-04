// Account management module

use crate::miden_client::{LendingClient, AccountStorageMode, AccountId};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

// This module handles:
// - Creating user lending accounts
// - Deploying lending pool accounts
// - Deploying price oracle accounts
// - Managing account state

pub struct AccountManager {
    client: LendingClient,
}

impl AccountManager {
    pub async fn new(rpc_endpoint: &str, store_path: &Path) -> Result<Self> {
        let client = LendingClient::new(rpc_endpoint, store_path.to_path_buf()).await?;
        Ok(Self { client })
    }

    /// Create a user lending account
    pub async fn create_user_account(&mut self, storage_mode: &str) -> Result<String> {
        // Load user lending account code
        let account_code = fs::read_to_string("../accounts/user_lending.masm")
            .context("Failed to read user lending account code")?;

        let mode = match storage_mode.to_lowercase().as_str() {
            "public" => AccountStorageMode::Public,
            "private" => AccountStorageMode::Private,
            _ => AccountStorageMode::Private,
        };

        let account_id = self.client.create_account(&account_code, mode).await?;

        Ok(format!("User account created: {}", account_id.to_hex()))
    }

    /// Deploy the lending pool account
    pub async fn deploy_lending_pool(&mut self) -> Result<String> {
        // Load lending pool account code
        let account_code = fs::read_to_string("../accounts/lending_pool.masm")
            .context("Failed to read lending pool account code")?;

        // Lending pool should be public
        let account_id = self.client
            .create_account(&account_code, AccountStorageMode::Public)
            .await?;

        Ok(format!("Lending pool deployed: {}", account_id.to_hex()))
    }

    /// Deploy the price oracle account
    pub async fn deploy_price_oracle(&mut self) -> Result<String> {
        // Load price oracle account code
        let account_code = fs::read_to_string("../accounts/price_oracle.masm")
            .context("Failed to read price oracle account code")?;

        // Oracle should be public
        let account_id = self.client
            .create_account(&account_code, AccountStorageMode::Public)
            .await?;

        Ok(format!("Price oracle deployed: {}", account_id.to_hex()))
    }

    /// Get account information
    pub async fn get_account_info(&self, account_id: &AccountId) -> Result<AccountInfo> {
        let account = self.client.get_account(account_id).await?;

        // Parse storage to extract account data
        // For user accounts: storage[0-2] = collateral, storage[3-5] = debt
        let collateral_usdc = if account.storage.len() > 0 {
            account.storage[0]
        } else {
            0
        };
        let collateral_dai = if account.storage.len() > 1 {
            account.storage[1]
        } else {
            0
        };
        let collateral_weth = if account.storage.len() > 2 {
            account.storage[2]
        } else {
            0
        };

        let debt_usdc = if account.storage.len() > 3 {
            account.storage[3]
        } else {
            0
        };
        let debt_dai = if account.storage.len() > 4 {
            account.storage[4]
        } else {
            0
        };
        let debt_weth = if account.storage.len() > 5 {
            account.storage[5]
        } else {
            0
        };

        Ok(AccountInfo {
            account_id: account_id.to_hex(),
            collateral_usdc,
            collateral_dai,
            collateral_weth,
            debt_usdc,
            debt_dai,
            debt_weth,
        })
    }
}

#[derive(Debug)]
pub struct AccountInfo {
    pub account_id: String,
    pub collateral_usdc: u64,
    pub collateral_dai: u64,
    pub collateral_weth: u64,
    pub debt_usdc: u64,
    pub debt_dai: u64,
    pub debt_weth: u64,
}

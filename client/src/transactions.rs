// Transaction building and execution module

use crate::miden_client::{LendingClient, AccountId, TransactionScriptBuilder};
use anyhow::{Context, Result};
use tracing::{info, debug};

/// Transaction builder and executor for lending protocol operations
pub struct TransactionBuilder {
    client: LendingClient,
    pool_account_id: AccountId,
    oracle_account_id: AccountId,
}

impl TransactionBuilder {
    /// Create a new transaction builder
    pub fn new(
        client: LendingClient,
        pool_account_id: AccountId,
        oracle_account_id: AccountId,
    ) -> Self {
        Self {
            client,
            pool_account_id,
            oracle_account_id,
        }
    }

    /// Deposit assets to the lending pool
    pub async fn deposit(&mut self, user_account_id: &AccountId, asset_id: u64, amount: u64) -> Result<()> {
        info!("Executing deposit: {} units of asset {}", amount, asset_id);

        // Build transaction script that calls lending_pool::deposit
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(&self.pool_account_id, "deposit")
            .build();

        debug!("Deposit script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to execute deposit transaction")?;

        if result.success {
            info!("Deposit successful. TX ID: {}", hex::encode(&result.tx_id));
        } else {
            anyhow::bail!("Deposit transaction failed");
        }

        Ok(())
    }

    /// Withdraw assets from the lending pool
    pub async fn withdraw(&mut self, user_account_id: &AccountId, asset_id: u64, amount: u64) -> Result<()> {
        info!("Executing withdrawal: {} units of asset {}", amount, asset_id);

        // Build transaction script that calls lending_pool::withdraw
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(&self.pool_account_id, "withdraw")
            .build();

        debug!("Withdraw script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to execute withdraw transaction")?;

        if result.success {
            info!("Withdrawal successful. TX ID: {}", hex::encode(&result.tx_id));
        } else {
            anyhow::bail!("Withdrawal transaction failed");
        }

        Ok(())
    }

    /// Supply collateral to user's lending account
    pub async fn supply_collateral(&mut self, user_account_id: &AccountId, asset_id: u64, amount: u64) -> Result<()> {
        info!("Supplying collateral: {} units of asset {}", amount, asset_id);

        // Build transaction script that calls user_lending::supply_collateral
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(user_account_id, "supply_collateral")
            .build();

        debug!("Supply collateral script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to execute supply collateral transaction")?;

        if result.success {
            info!("Collateral supplied successfully. TX ID: {}", hex::encode(&result.tx_id));
        } else {
            anyhow::bail!("Supply collateral transaction failed");
        }

        Ok(())
    }

    /// Borrow assets from the lending pool
    pub async fn borrow(&mut self, user_account_id: &AccountId, asset_id: u64, amount: u64) -> Result<()> {
        info!("Executing borrow: {} units of asset {}", amount, asset_id);

        // First, calculate health factor to verify borrowing is safe
        // In production, this would involve:
        // 1. Get user's collateral value (from user account storage)
        // 2. Get current debt value
        // 3. Calculate health factor
        // 4. Assert health factor >= 1.0 after borrow

        // Build transaction script that calls lending_pool::borrow
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(&self.pool_account_id, "borrow")
            .build();

        debug!("Borrow script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to execute borrow transaction")?;

        if result.success {
            info!("Borrow successful. TX ID: {}", hex::encode(&result.tx_id));

            // Update user's debt tracking
            self.record_borrow_in_user_account(user_account_id, asset_id, amount).await?;
        } else {
            anyhow::bail!("Borrow transaction failed");
        }

        Ok(())
    }

    /// Repay borrowed assets
    pub async fn repay(&mut self, user_account_id: &AccountId, asset_id: u64, amount: u64) -> Result<()> {
        info!("Executing repayment: {} units of asset {}", amount, asset_id);

        // Build transaction script that calls lending_pool::repay
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(&self.pool_account_id, "repay")
            .build();

        debug!("Repay script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to execute repay transaction")?;

        if result.success {
            info!("Repayment successful. TX ID: {}", hex::encode(&result.tx_id));

            // Update user's debt tracking
            self.record_repayment_in_user_account(user_account_id, asset_id, amount).await?;
        } else {
            anyhow::bail!("Repayment transaction failed");
        }

        Ok(())
    }

    /// Execute a liquidation
    pub async fn liquidate(
        &mut self,
        liquidator_account_id: &AccountId,
        _borrower_account_id: &AccountId,
        collateral_asset_id: u64,
        debt_asset_id: u64,
        debt_to_cover: u64,
    ) -> Result<()> {
        info!(
            "Executing liquidation: covering {} units of debt asset {} for borrower",
            debt_to_cover, debt_asset_id
        );

        // Verify borrower is undercollateralized
        // This would check health factor < 1.0

        // Build liquidation transaction
        // In production, this would create a liquidation note
        let script = format!(
            "begin\n    push.{}\n    push.{}\n    push.{}\n    # Liquidation logic here\nend",
            debt_to_cover, debt_asset_id, collateral_asset_id
        );

        debug!("Liquidation script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, liquidator_account_id)
            .await
            .context("Failed to execute liquidation transaction")?;

        if result.success {
            info!("Liquidation successful. TX ID: {}", hex::encode(&result.tx_id));
        } else {
            anyhow::bail!("Liquidation transaction failed");
        }

        Ok(())
    }

    /// Update asset price in the oracle
    pub async fn update_price(&mut self, admin_account_id: &AccountId, asset_id: u64, price: u64) -> Result<()> {
        info!("Updating price for asset {}: {}", asset_id, price);

        // Build transaction script that calls price_oracle::update_asset_price
        let script = TransactionScriptBuilder::new()
            .push(price)
            .push(asset_id)
            .call_procedure(&self.oracle_account_id, "update_asset_price")
            .build();

        debug!("Update price script:\n{}", script);

        // Execute transaction
        let result = self.client
            .execute_transaction(&script, admin_account_id)
            .await
            .context("Failed to execute price update transaction")?;

        if result.success {
            info!("Price updated successfully. TX ID: {}", hex::encode(&result.tx_id));
        } else {
            anyhow::bail!("Price update transaction failed");
        }

        Ok(())
    }

    /// Get reserve data from lending pool
    pub async fn get_reserve_data(&mut self, asset_id: u64) -> Result<ReserveData> {
        info!("Fetching reserve data for asset {}", asset_id);

        // Build query script that calls lending_pool::get_reserve_data
        let script = TransactionScriptBuilder::new()
            .push(asset_id)
            .call_procedure(&self.pool_account_id, "get_reserve_data")
            .build();

        debug!("Get reserve data script:\n{}", script);

        // Execute transaction (read-only)
        let _result = self.client
            .execute_transaction(&script, &self.pool_account_id)
            .await
            .context("Failed to query reserve data")?;

        // In production, parse the transaction output to extract:
        // [total_liquidity, total_borrowed, liquidity_rate, borrow_rate]

        // Placeholder values
        Ok(ReserveData {
            asset_id,
            total_liquidity: 0,
            total_borrowed: 0,
            liquidity_rate: 0,
            borrow_rate: 0,
        })
    }

    /// Get asset price from oracle
    pub async fn get_price(&mut self, asset_id: u64) -> Result<u64> {
        info!("Fetching price for asset {}", asset_id);

        // Build query script that calls price_oracle::get_asset_price
        let script = TransactionScriptBuilder::new()
            .push(asset_id)
            .call_procedure(&self.oracle_account_id, "get_asset_price")
            .build();

        debug!("Get price script:\n{}", script);

        // Execute transaction (read-only)
        let _result = self.client
            .execute_transaction(&script, &self.oracle_account_id)
            .await
            .context("Failed to query asset price")?;

        // In production, parse the transaction output
        // Placeholder
        Ok(100000000) // $1.00 with 8 decimals
    }

    /// Calculate user's health factor
    pub async fn calculate_health_factor(&mut self, user_account_id: &AccountId) -> Result<u64> {
        info!("Calculating health factor for user");

        // Build script that calls user_lending::calculate_health_factor
        let script = format!(
            "begin\n    # Get collateral and debt from storage\n    # Calculate health factor\nend"
        );

        debug!("Health factor script:\n{}", script);

        // Execute transaction (read-only)
        let _result = self.client
            .execute_transaction(&script, user_account_id)
            .await
            .context("Failed to calculate health factor")?;

        // In production, parse the result
        // Placeholder: return 1.5 (healthy)
        Ok(15000) // 1.5 in basis points (10000 = 1.0)
    }

    // Helper functions

    /// Record borrow in user's account storage
    async fn record_borrow_in_user_account(
        &mut self,
        user_account_id: &AccountId,
        asset_id: u64,
        amount: u64,
    ) -> Result<()> {
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(user_account_id, "record_borrow")
            .build();

        self.client
            .execute_transaction(&script, user_account_id)
            .await?;

        Ok(())
    }

    /// Record repayment in user's account storage
    async fn record_repayment_in_user_account(
        &mut self,
        user_account_id: &AccountId,
        asset_id: u64,
        amount: u64,
    ) -> Result<()> {
        let script = TransactionScriptBuilder::new()
            .push(amount)
            .push(asset_id)
            .call_procedure(user_account_id, "record_repayment")
            .build();

        self.client
            .execute_transaction(&script, user_account_id)
            .await?;

        Ok(())
    }
}

/// Reserve data structure
#[derive(Debug, Clone)]
pub struct ReserveData {
    pub asset_id: u64,
    pub total_liquidity: u64,
    pub total_borrowed: u64,
    pub liquidity_rate: u64,
    pub borrow_rate: u64,
}

impl ReserveData {
    pub fn utilization_rate(&self) -> f64 {
        if self.total_liquidity == 0 {
            0.0
        } else {
            (self.total_borrowed as f64) / (self.total_liquidity as f64)
        }
    }

    pub fn available_liquidity(&self) -> u64 {
        self.total_liquidity.saturating_sub(self.total_borrowed)
    }
}

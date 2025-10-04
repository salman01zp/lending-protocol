// Account Components for Miden Lending Protocol
// These components wrap MASM account logic with Rust builders

pub mod lending_pool;
pub mod user_lending;
pub mod price_oracle;

#[cfg(any(feature = "testing", test))]
pub mod testing {
    pub use super::lending_pool::create_lending_pool_account_builder;
    pub use super::user_lending::create_user_lending_account_builder;
    pub use super::price_oracle::create_price_oracle_account_builder;
}

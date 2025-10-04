use miden_objects::{
    account::{
        Account, AccountBuilder, AccountComponent, AccountStorageMode, AccountType, StorageSlot,
    },
    utils::sync::LazyLock,
    AccountError, Felt, FieldElement, Word,
};

/// Compiled MASM library for price oracle
static PRICE_ORACLE_LIBRARY_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/assets/contracts/price_oracle.masl"));

static PRICE_ORACLE_LIBRARY: LazyLock<miden_objects::assembly::Library> =
    LazyLock::new(|| {
        miden_objects::utils::Deserializable::read_from_bytes(PRICE_ORACLE_LIBRARY_BYTES)
            .expect("failed to deserialize price oracle library")
    });

fn price_oracle_library() -> miden_objects::assembly::Library {
    PRICE_ORACLE_LIBRARY.clone()
}

/// PriceOracle Account Component
///
/// Maintains price feeds for supported assets.
/// Storage slots contain price data for USDC, DAI, and WETH.
pub struct PriceOracleAccount {
    /// Asset prices stored as Word
    /// Storage slot 0: USDC price (8 decimals)
    /// Storage slot 1: DAI price (8 decimals)
    /// Storage slot 2: WETH price (8 decimals)
    prices: Vec<Word>,

    /// Last update timestamps
    /// Storage slot 3: last update timestamp
    last_update: Word,
}

impl PriceOracleAccount {
    /// Create a new price oracle with default prices
    /// USDC: $1.00, DAI: $1.00, WETH: $2500.00 (all with 8 decimals)
    pub fn new() -> Self {
        let default_prices = vec![
            Word::new([Felt::new(100000000), Felt::ZERO, Felt::ZERO, Felt::ZERO]), // USDC: $1.00
            Word::new([Felt::new(100000000), Felt::ZERO, Felt::ZERO, Felt::ZERO]), // DAI: $1.00
            Word::new([Felt::new(250000000000), Felt::ZERO, Felt::ZERO, Felt::ZERO]), // WETH: $2500.00
        ];

        Self {
            prices: default_prices,
            last_update: Word::default(),
        }
    }

    /// Create with custom prices
    pub fn with_prices(prices: Vec<Word>) -> Self {
        Self {
            prices,
            last_update: Word::default(),
        }
    }

    /// Set a specific asset price
    pub fn set_price(&mut self, asset_index: usize, price: u64) {
        if asset_index < self.prices.len() {
            self.prices[asset_index] = Word::new([Felt::new(price), Felt::ZERO, Felt::ZERO, Felt::ZERO]);
        }
    }
}

impl Default for PriceOracleAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl From<PriceOracleAccount> for AccountComponent {
    fn from(oracle: PriceOracleAccount) -> Self {
        let mut storage_slots = Vec::new();

        // Add price slots
        for price_word in oracle.prices {
            storage_slots.push(StorageSlot::Value(price_word));
        }

        // Add last update timestamp
        storage_slots.push(StorageSlot::Value(oracle.last_update));

        AccountComponent::new(price_oracle_library(), storage_slots)
            .expect("price oracle component should be valid")
            .with_supported_type(AccountType::RegularAccountUpdatableCode)
    }
}

/// Creates a price oracle account builder
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `account_storage_mode` - Public or Private storage mode
pub fn create_price_oracle_account_builder(
    init_seed: [u8; 32],
    account_storage_mode: AccountStorageMode,
) -> Result<AccountBuilder, AccountError> {
    Ok(AccountBuilder::new(init_seed)
        .account_type(AccountType::RegularAccountUpdatableCode)
        .storage_mode(account_storage_mode)
        .with_component(PriceOracleAccount::new()))
}

/// Creates a price oracle account with authentication
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `account_storage_mode` - Public or Private storage mode
/// * `auth_scheme` - Authentication scheme
///
/// Returns the created account and its seed
#[cfg(any(feature = "testing", test))]
pub fn create_price_oracle_account(
    init_seed: [u8; 32],
    account_storage_mode: AccountStorageMode,
    auth_scheme: miden_lib::AuthScheme,
) -> Result<(Account, Word), AccountError> {
    use miden_lib::account::auth::{AuthRpoFalcon512Acl, AuthRpoFalcon512AclConfig};

    let auth_component: AuthRpoFalcon512Acl = match auth_scheme {
        miden_lib::AuthScheme::RpoFalcon512 { pub_key } => {
            Ok(AuthRpoFalcon512Acl::new(
                pub_key,
                AuthRpoFalcon512AclConfig::new(),
            )?)
        }
        _ => Err(AccountError::other("unsupported auth scheme")),
    }?;

    let (account, account_seed) = create_price_oracle_account_builder(init_seed, account_storage_mode)?
        .with_auth_component(auth_component)
        .build()?;

    Ok((account, account_seed))
}

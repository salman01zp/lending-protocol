use miden_objects::{
    account::{
        Account, AccountBuilder, AccountComponent, AccountStorageMode, AccountType, StorageSlot,
    },
    utils::sync::LazyLock,
    AccountError, Felt, FieldElement, Word,
};

/// Compiled MASM library for lending pool
static LENDING_POOL_LIBRARY_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/assets/contracts/lending_pool.masl"));

static LENDING_POOL_LIBRARY: LazyLock<miden_objects::assembly::Library> =
    LazyLock::new(|| {
        miden_objects::utils::Deserializable::read_from_bytes(LENDING_POOL_LIBRARY_BYTES)
            .expect("failed to deserialize lending pool library")
    });

fn lending_pool_library() -> miden_objects::assembly::Library {
    LENDING_POOL_LIBRARY.clone()
}

/// LendingPool Account Component
///
/// Manages liquidity pools for multiple assets (USDC, DAI, WETH).
/// Storage slots contain reserve data including total liquidity, borrowed amounts,
/// and interest rates for each asset.
pub struct LendingPoolAccount {
    /// Reserve data for each asset stored as Word
    /// Each reserve uses multiple storage slots for:
    /// - total_liquidity
    /// - total_borrowed
    /// - liquidity_rate
    /// - borrow_rate
    /// - last_update_timestamp
    /// - liquidity_index
    /// - borrow_index
    usdc_reserve: Vec<Word>,
    dai_reserve: Vec<Word>,
    weth_reserve: Vec<Word>,
}

impl LendingPoolAccount {
    /// Create a new lending pool with empty reserves
    pub fn new() -> Self {
        // Initialize with empty reserves
        Self {
            usdc_reserve: vec![Word::default(); 7],
            dai_reserve: vec![Word::default(); 6],
            weth_reserve: vec![Word::default(); 6],
        }
    }

    /// Create with custom initial reserve values
    pub fn with_reserves(
        usdc_reserve: Vec<Word>,
        dai_reserve: Vec<Word>,
        weth_reserve: Vec<Word>,
    ) -> Self {
        Self {
            usdc_reserve,
            dai_reserve,
            weth_reserve,
        }
    }
}

impl Default for LendingPoolAccount {
    fn default() -> Self {
        Self::new()
    }
}

impl From<LendingPoolAccount> for AccountComponent {
    fn from(pool: LendingPoolAccount) -> Self {
        let mut storage_slots = Vec::new();

        // Add USDC reserve slots (0-6)
        for word in pool.usdc_reserve {
            storage_slots.push(StorageSlot::Value(word));
        }

        // Add DAI reserve slots (7-12)
        for word in pool.dai_reserve {
            storage_slots.push(StorageSlot::Value(word));
        }

        // Add WETH reserve slots (13-18)
        for word in pool.weth_reserve {
            storage_slots.push(StorageSlot::Value(word));
        }

        AccountComponent::new(lending_pool_library(), storage_slots)
            .expect("lending pool component should be valid")
            .with_supported_type(AccountType::RegularAccountUpdatableCode)
    }
}

/// Creates a lending pool account builder
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `account_storage_mode` - Public or Private storage mode
pub fn create_lending_pool_account_builder(
    init_seed: [u8; 32],
    account_storage_mode: AccountStorageMode,
) -> Result<AccountBuilder, AccountError> {
    Ok(AccountBuilder::new(init_seed)
        .account_type(AccountType::RegularAccountUpdatableCode)
        .storage_mode(account_storage_mode)
        .with_component(LendingPoolAccount::new()))
}

/// Creates a lending pool account with authentication
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `account_storage_mode` - Public or Private storage mode
/// * `auth_scheme` - Authentication scheme (e.g., RpoFalcon512)
///
/// Returns the created account and its seed
#[cfg(any(feature = "testing", test))]
pub fn create_lending_pool_account(
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

    let (account, account_seed) = create_lending_pool_account_builder(init_seed, account_storage_mode)?
        .with_auth_component(auth_component)
        .build()?;

    Ok((account, account_seed))
}

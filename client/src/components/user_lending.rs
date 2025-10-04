use miden_objects::{
    account::{
        Account, AccountBuilder, AccountComponent, AccountStorageMode, AccountType, StorageSlot,
    },
    utils::sync::LazyLock,
    AccountError, Felt, FieldElement, Word,
};

/// Compiled MASM library for user lending account
static USER_LENDING_LIBRARY_BYTES: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/assets/contracts/user_lending.masl"));

static USER_LENDING_LIBRARY: LazyLock<miden_objects::assembly::Library> =
    LazyLock::new(|| {
        miden_objects::utils::Deserializable::read_from_bytes(USER_LENDING_LIBRARY_BYTES)
            .expect("failed to deserialize user lending library")
    });

fn user_lending_library() -> miden_objects::assembly::Library {
    USER_LENDING_LIBRARY.clone()
}

/// UserLending Account Component
///
/// Tracks individual user positions including collateral and debt.
/// Storage slots contain user-specific data for all supported assets.
pub struct UserLendingAccount {
    /// User collateral amounts for each asset
    /// Storage slots 0-2: collateral (USDC, DAI, WETH)
    collateral: Word,

    /// User debt amounts for each asset
    /// Storage slots 3-5: debt (USDC, DAI, WETH)
    debt: Word,

    /// User's lending pool account ID reference
    /// Storage slot 6: pool account ID
    pool_account_id: Word,
}

impl UserLendingAccount {
    /// Create a new user lending account with zero balances
    pub fn new(pool_account_id: Word) -> Self {
        Self {
            collateral: Word::default(),
            debt: Word::default(),
            pool_account_id,
        }
    }

    /// Create with initial collateral and debt
    pub fn with_positions(
        collateral: Word,
        debt: Word,
        pool_account_id: Word,
    ) -> Self {
        Self {
            collateral,
            debt,
            pool_account_id,
        }
    }
}

impl From<UserLendingAccount> for AccountComponent {
    fn from(user: UserLendingAccount) -> Self {
        let storage_slots = vec![
            StorageSlot::Value(user.collateral),
            StorageSlot::Value(user.debt),
            StorageSlot::Value(user.pool_account_id),
        ];

        AccountComponent::new(user_lending_library(), storage_slots)
            .expect("user lending component should be valid")
            .with_supported_type(AccountType::RegularAccountUpdatableCode)
    }
}

/// Creates a user lending account builder
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `pool_account_id` - The lending pool account ID this user will interact with
/// * `account_storage_mode` - Public or Private storage mode
pub fn create_user_lending_account_builder(
    init_seed: [u8; 32],
    pool_account_id: Word,
    account_storage_mode: AccountStorageMode,
) -> Result<AccountBuilder, AccountError> {
    Ok(AccountBuilder::new(init_seed)
        .account_type(AccountType::RegularAccountUpdatableCode)
        .storage_mode(account_storage_mode)
        .with_component(UserLendingAccount::new(pool_account_id)))
}

/// Creates a user lending account with authentication
///
/// # Arguments
/// * `init_seed` - Random seed for account ID generation
/// * `pool_account_id` - The lending pool account ID
/// * `account_storage_mode` - Public or Private storage mode
/// * `auth_scheme` - Authentication scheme
///
/// Returns the created account and its seed
#[cfg(any(feature = "testing", test))]
pub fn create_user_lending_account(
    init_seed: [u8; 32],
    pool_account_id: Word,
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

    let (account, account_seed) = create_user_lending_account_builder(
        init_seed,
        pool_account_id,
        account_storage_mode,
    )?
    .with_auth_component(auth_component)
    .build()?;

    Ok((account, account_seed))
}

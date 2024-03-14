use soroban_sdk::{contracttype, unwrap::UnwrapOptimized, Address, BytesN, Env, Symbol};

// @dev: This contract is not expected to be used often, so we can use a higher bump amount
pub(crate) const LEDGER_THRESHOLD: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP: u32 = 241920; // ~ 14 days

const IS_INIT_KEY: &str = "IsInit";
const ADMIN_KEY: &str = "Admin";

#[derive(Clone)]
#[contracttype]
pub enum TreasuryFactoryDataKey {
    Contracts(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct TreasuryInitMeta {
    pub treasury_hash: BytesN<32>,
    pub pool_factory: Address,
}

/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_BUMP);
}

/// Check if the contract has been initialized
pub fn get_is_init(e: &Env) -> bool {
    e.storage().instance().has(&Symbol::new(e, IS_INIT_KEY))
}

/// Set the contract as initialized
pub fn set_is_init(e: &Env) {
    e.storage()
        .instance()
        .set::<Symbol, bool>(&Symbol::new(e, IS_INIT_KEY), &true);
}

/// Fetch the current admin Address
pub fn get_admin(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, ADMIN_KEY))
        .unwrap_optimized()
}

/// Set a new admin
///
/// ### Arguments
/// * `new_admin` - The Address for the admin
pub fn set_admin(e: &Env, new_admin: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, ADMIN_KEY), new_admin);
}

/// Fetch the pool initialization metadata
pub fn get_pool_init_meta(e: &Env) -> TreasuryInitMeta {
    e.storage()
        .instance()
        .get::<Symbol, TreasuryInitMeta>(&Symbol::new(e, "TreasuryMeta"))
        .unwrap_optimized()
}

/// Set the pool initialization metadata
///
/// ### Arguments
/// * `pool_init_meta` - The metadata to initialize pools
pub fn set_pool_init_meta(e: &Env, pool_init_meta: &TreasuryInitMeta) {
    e.storage()
        .instance()
        .set::<Symbol, TreasuryInitMeta>(&Symbol::new(e, "TreasuryMeta"), pool_init_meta)
}

/// Check if a given contract_id was deployed by the factory
///
/// ### Arguments
/// * `contract_id` - The contract_id to check
pub fn is_deployed(e: &Env, contract_id: &Address) -> bool {
    let key = TreasuryFactoryDataKey::Contracts(contract_id.clone());
    if let Some(result) = e
        .storage()
        .persistent()
        .get::<TreasuryFactoryDataKey, bool>(&key)
    {
        e.storage()
            .persistent()
            .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
        result
    } else {
        false
    }
}

/// Set a contract_id as having been deployed by the factory
///
/// ### Arguments
/// * `contract_id` - The contract_id that was deployed by the factory
pub fn set_deployed(e: &Env, contract_id: &Address) {
    let key = TreasuryFactoryDataKey::Contracts(contract_id.clone());
    e.storage()
        .persistent()
        .set::<TreasuryFactoryDataKey, bool>(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}
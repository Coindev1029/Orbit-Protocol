use soroban_sdk::{contracttype, unwrap::UnwrapOptimized, Address, BytesN, Env, Symbol};

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

const LEDGER_THRESHOLD_PERSISTENT: u32 = ONE_DAY_LEDGERS * 100; // ~ 100 days
const LEDGER_BUMP_PERSISTENT: u32 = LEDGER_THRESHOLD_PERSISTENT + 20 * ONE_DAY_LEDGERS; // ~ 120 days

const ADMIN_KEY: &str = "Admin";
const BRIDGE_ORACLE_KEY: &str = "Oracle";
const SOROSWAP_KEY: &str = "Soroswap";
const TREASURY_META_KEY: &str = "TreasuryMeta";

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
        .extend_ttl(LEDGER_BUMP_INSTANCE, LEDGER_BUMP_INSTANCE);
}

/// Check if the contract has been initialized
pub fn is_init(e: &Env) -> bool { e.storage().instance().has(&Symbol::new(e, ADMIN_KEY)) }

/// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
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

/// Fetch the current bridge oracle Address
///
/// ### Panics
/// If the bridge oracle does not exist
pub fn get_bridge_oracle(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, BRIDGE_ORACLE_KEY))
        .unwrap_optimized()
}

/// Set a new bridge oracle
///
/// ### Arguments
/// * `new_bridge_oracle` - The Address for the bridge oracle
pub fn set_bridge_oracle(e: &Env, new_bridge_oracle: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, BRIDGE_ORACLE_KEY), new_bridge_oracle);
}

/// Fetch the current soroswap Address
///
/// ### Panics
/// If the bridge soroswap does not exist
pub fn get_soroswap(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, SOROSWAP_KEY))
        .unwrap_optimized()
}

/// Set a new soroswap
///
/// ### Arguments
/// * `new_soroswap_address` - The Address for the soroswap
// pub fn set_soroswap(e: &Env, new_soroswap_address: &Address) {
//     e.storage()
//         .instance()
//         .set::<Symbol, Address>(&Symbol::new(e, SOROSWAP_KEY), new_soroswap_address);
// }

/// Fetch the pool initialization metadata
pub fn get_pool_init_meta(e: &Env) -> TreasuryInitMeta {
    e.storage()
        .instance()
        .get::<Symbol, TreasuryInitMeta>(&Symbol::new(e, TREASURY_META_KEY))
        .unwrap_optimized()
}

/// Set the pool initialization metadata
///
/// ### Arguments
/// * `pool_init_meta` - The metadata to initialize pools
pub fn set_pool_init_meta(e: &Env, pool_init_meta: &TreasuryInitMeta) {
    e.storage()
        .instance()
        .set::<Symbol, TreasuryInitMeta>(&Symbol::new(e, TREASURY_META_KEY), pool_init_meta)
}

/// Check if a given token_address was deployed by the factory
///
/// ### Arguments
/// * `contract_id` - The contract_id to check
pub fn is_deployed(e: &Env, treasury_address: &Address) -> bool {
    let key = TreasuryFactoryDataKey::Contracts(treasury_address.clone());
    if let Some(result) = e
        .storage()
        .persistent()
        .get::<TreasuryFactoryDataKey, bool>(&key)
    {
        e.storage()
            .persistent()
            .extend_ttl(&key, LEDGER_THRESHOLD_PERSISTENT, LEDGER_BUMP_PERSISTENT);
        result
    } else {
        false
    }
}

/// Set a contract_id as having been deployed by the factory
///
/// ### Arguments
/// * `treasury_address` - The treasury_address that was deployed
pub fn set_deployed(e: &Env, treasury_address: &Address) {
    let key = TreasuryFactoryDataKey::Contracts(treasury_address.clone());
    e.storage()
        .persistent()
        .set::<TreasuryFactoryDataKey, bool>(&key, &true);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD_PERSISTENT, LEDGER_BUMP_PERSISTENT);
}
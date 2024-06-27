use soroban_sdk::{Address, contracttype, Env, Symbol};
use soroban_sdk::unwrap::UnwrapOptimized;

pub(crate) const LEDGER_THRESHOLD_SHARED: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP_SHARED: u32 = 241920; // ~ 14 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ADMIN,
    BLENDPOOL(Address), // mapping token address to the blend pool addres
    PEGKEEPER,
    BRIDGEORACLE,
}

/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

/// Check if the contract has been initialized
pub fn is_init(e: &Env) -> bool { e.storage().instance().has(&DataKey::ADMIN) }

/// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
pub fn get_admin(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::ADMIN)
        .unwrap_optimized()
}

/// Set a new admin
///
/// ### Arguments
/// * `new_admin` - The Address for the admin
pub fn set_admin(e: &Env, new_admin: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::ADMIN, new_admin);
}

// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
pub fn get_pegkeeper(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::PEGKEEPER)
        .unwrap_optimized()
}

/// Set a new admin
///
/// ### Arguments
/// * `new_admin` - The Address for the admin
pub fn set_pegkeeper(e: &Env, new_pegkeeper: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::PEGKEEPER, new_pegkeeper);
}

/// Fetch the current treasury Address depending on token address
///
/// ### Panics
/// If the treasury does not exist
pub fn get_blend_pool(e: &Env, token_address: &Address) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::BLENDPOOL(token_address.clone()))
        .unwrap_optimized()
}

/// Set the treasury Address depending on token address
///
/// ### Arguments
/// * `token_address` - token address of treasury, `treasury_address` - The Address for the treasury
pub fn set_blend_pool(e: &Env, token_address: &Address, blend_pool: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::BLENDPOOL(token_address.clone()), blend_pool);
}

/// Fetch the current bridge oracle
///
/// ### Panics
/// If the bridge oracle does not exist
pub fn get_bridge_oracle(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::BRIDGEORACLE)
        .unwrap_optimized()
}

/// Set the bridge oracle
///
/// ### Arguments
/// * `bridge_oracle` - The Address for the bridge oracle
pub fn set_bridge_oracle(e: &Env, bridge_oracle: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::BRIDGEORACLE, bridge_oracle);
}
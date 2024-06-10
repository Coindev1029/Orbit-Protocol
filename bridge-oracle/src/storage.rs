use soroban_sdk::{Address, Env, Symbol, contracttype};
use sep_40_oracle::Asset;
use soroban_sdk::unwrap::UnwrapOptimized;

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

#[derive(Clone)]
#[contracttype]
pub enum BridgeOracleDataKey {
    ToAsset(Asset),
}

const ADMIN_KEY: &str = "Admin";
const ORACLE_KEY: &str = "Oracle";

/// Bump the instance rent for the contract
pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_INSTANCE, LEDGER_BUMP_INSTANCE);
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

/// Fetch the asset to convert to
///
/// ### Arguments
/// * `asset` - The asset to convert from
pub fn get_to_asset(env: &Env, asset: &Asset) -> Asset {
    env.storage()
        .instance()
        .get::<BridgeOracleDataKey, Asset>(&BridgeOracleDataKey::ToAsset(asset.clone()))
        .unwrap_or_else(|| asset.clone())
}

/// Set the asset to convert to
///
/// ### Arguments
/// * `asset` - The asset to convert from
/// * `to` - The asset to convert to
pub fn set_to_asset(env: &Env, asset: &Asset, to: &Asset) {
    env.storage()
        .instance()
        .set::<BridgeOracleDataKey, Asset>(&BridgeOracleDataKey::ToAsset(asset.clone()), to);
}

/// Fetch the current oracle Address
///
/// ### Panics
/// If the oracle does not exist
pub fn get_oracle(env: &Env) -> Address {
    env.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(env, ORACLE_KEY))
        .unwrap_optimized()
}

/// Set a new oracle
///
/// ### Arguments
/// * `address` - The Address for the oracle
pub fn set_oracle(env: &Env, address: &Address) {
    env.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(env, ORACLE_KEY), address);
}
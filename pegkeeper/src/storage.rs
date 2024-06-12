use soroban_sdk::{Address, Env, Symbol, symbol_short, unwrap::UnwrapOptimized, contracttype};

use crate::dependencies::treasury;

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ADMIN,
    TREASURY(Address), // mapping token address to treasury addres
    BLEND,
    SOROSWAP,
    BALANCE
}
/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_INSTANCE, LEDGER_BUMP_INSTANCE);
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

/// Fetch the current treasury Address depending on token address
///
/// ### Panics
/// If the treasury does not exist
pub fn get_treasury(e: &Env, token_address: Address) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::TREASURY(token_address))
        .unwrap_optimized()
}

/// Set the treasury Address depending on token address
///
/// ### Arguments
/// * `token_address` - token address of treasury, `treasury_address` - The Address for the treasury
pub fn set_treasury(e: &Env, token_address: Address, treasury_address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::TREASURY(token_address), treasury_address);
}

/// Fetch the current blend Address
///
/// ### Panics
/// If the blend does not exist
pub fn get_blend(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::BLEND)
        .unwrap_optimized()
}

/// Set the blend Address
///
/// ### Arguments
/// * `blend` - The Address for the blend pool
pub fn set_blend(e: &Env, blend: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::BLEND, blend);
}

/// Fetch the current soroswap Address
///
/// ### Panics
/// If the soroswap does not exist
pub fn get_soroswap(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::SOROSWAP)
        .unwrap_optimized()
}

/// Set the soroswap Address
///
/// ### Arguments
/// * `soroswap` - The Address of the soroswap
pub fn set_soroswap(e: &Env, soroswap: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::SOROSWAP, soroswap);
}

/// Fetch the current balance
///
pub fn get_balance(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&DataKey::BALANCE)
        .unwrap_optimized()
}

/// Set the token balance
///
/// ### Arguments
/// * `balance` - The balance of the contract
pub fn set_balance(e: &Env, balance: &i128) {
    e.storage()
        .instance()
        .set(&DataKey::BALANCE, balance);
}
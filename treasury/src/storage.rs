use soroban_sdk::{Address, Env, Symbol};
use soroban_sdk::unwrap::UnwrapOptimized;

pub(crate) const LEDGER_THRESHOLD_SHARED: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP_SHARED: u32 = 241920; // ~ 14 days

const ADMIN_KEY: &str = "Admin";
const BLEND_KEY: &str = "Blend";
const TOKEN_KEY: &str = "Token";
const TOKEN_SUPPLY_KEY: &str = "TokenSupply";

/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

/********** Admin **********/

// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
pub fn get_admin(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, ADMIN_KEY))
        .unwrap()
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

/********** Token **********/

/// Fetch the current token Address
///
/// ### Panics
/// If the token does not exist
pub fn get_token(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, TOKEN_KEY))
        .unwrap_optimized()
}

/// Set the token Address
///
/// ### Arguments
/// * `token` - The Address for the token
pub fn set_token(e: &Env, token: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, TOKEN_KEY), token);
}

/********** Token Supply **********/

/// Fetch the current token supply
///
/// ### Panics
/// If the token supply does not exist
pub fn get_token_supply(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&Symbol::new(e, TOKEN_SUPPLY_KEY))
        .unwrap_optimized()
}

/// Set the token supply
///
/// ### Arguments
/// * `supply` - The new supply
pub fn set_token_supply(e: &Env, supply: &i128) {
    e.storage()
        .instance()
        .set::<Symbol, i128>(&Symbol::new(e, TOKEN_SUPPLY_KEY), supply);
}

/********** Blend **********/

/// Fetch the current blend Address
///
/// ### Panics
/// If the blend does not exist
pub fn get_blend(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, BLEND_KEY))
        .unwrap_optimized()
}

/// Set the blend Address
///
/// ### Arguments
/// * `blend` - The Address for the blend pool
pub fn set_blend(e: &Env, blend: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, BLEND_KEY), blend);
}

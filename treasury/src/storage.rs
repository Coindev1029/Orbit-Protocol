use soroban_sdk::{Address, Env, Symbol, unwrap::UnwrapOptimized};

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

const ADMIN_KEY: &str = "Admin";
const BLEND_KEY: &str = "Blend";
const TOKEN_KEY: &str = "Token";
const SUPPLY_KEY: &str = "Supply";

/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
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

/// Fetch the current token Address
///
/// ### Panics
/// If the token does not exist
pub fn get_token(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, TOKEN_KEY))
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

/// Fetch the current token supply
///
/// ### Panics
/// If the token supply does not exist
pub fn get_token_supply(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get::<Symbol, i128>(&Symbol::new(e, SUPPLY_KEY))
        .unwrap_optimized()
}

/// Set the token supply
///
/// ### Arguments
/// * `supply` - The new supply
pub fn set_token_supply(e: &Env, supply: &i128) {
    e.storage()
        .instance()
        .set::<Symbol, i128>(&Symbol::new(e, SUPPLY_KEY), supply);
}

/// Fetch the current blend Address
///
/// ### Panics
/// If the blend does not exist
pub fn get_blend(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, BLEND_KEY))
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

use soroban_sdk::{Address, Env, Symbol};
use soroban_sdk::unwrap::UnwrapOptimized;

pub(crate) const LEDGER_THRESHOLD_SHARED: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP_SHARED: u32 = 241920; // ~ 14 days

const ADMIN_KEY: &str = "Admin";
const BLEND_KEY: &str = "Blend";
const SOROSWAP_KEY: &str = "Soroswap";
const TOKEN_KEY: &str = "Token";
const TOKEN_SUPPLY_KEY: &str = "TokenSupply";
const COLLATERAL_TOKEN_KEY: &str = "CollateralKey";
const PEGKEEPER_KEY: &str = "Pegkeeper";
const LOAN_FEE_KEY: &str = "LoanFee";

/// Bump the instance rent for the contract
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}

/// Check if the contract has been initialized
pub fn is_init(e: &Env) -> bool { e.storage().instance().has(&Symbol::new(e, ADMIN_KEY)) }

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

// Fetch the current admin Address
///
/// ### Panics
/// If the admin does not exist
pub fn get_pegkeeper(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, PEGKEEPER_KEY))
        .unwrap()
}

/// Set a new admin
///
/// ### Arguments
/// * `new_admin` - The Address for the admin
pub fn set_pegkeeper(e: &Env, new_pegkeeper: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, PEGKEEPER_KEY), new_pegkeeper);
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

/********** Collatera Token **********/

/// Fetch the current collateral token Address
///
/// ### Panics
/// If the collateral token does not exist
pub fn get_collateral_token_address(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, COLLATERAL_TOKEN_KEY))
        .unwrap_optimized()
}

/// Set the collateral token Address
///
/// ### Arguments
/// * `token` - The Address for the collateral token
pub fn set_collateral_token_address(e: &Env, token: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, COLLATERAL_TOKEN_KEY), token);
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

/// Fetch the current loan fee
///
/// ### Panics
/// If the loan fee does not exist
// pub fn get_loan_fee(e: &Env) -> i128 {
//     e.storage()
//         .instance()
//         .get(&Symbol::new(e, LOAN_FEE_KEY))
//         .unwrap_optimized()
// }

/// Set the loan fee
///
/// ### Arguments
/// * `loan fee` - The new loan fee
pub fn set_loan_fee(e: &Env, fee: &i128) {
    e.storage()
        .instance()
        .set::<Symbol, i128>(&Symbol::new(e, LOAN_FEE_KEY), fee);
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

/********** Soroswap **********/

/// Fetch the current soroswap Address
///
/// ### Panics
/// If the soroswap does not exist
pub fn get_soroswap(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&Symbol::new(e, SOROSWAP_KEY))
        .unwrap_optimized()
}

/// Set the soroswap Address
///
/// ### Arguments
/// * `soroswap` - The Address for the soroswap
pub fn set_soroswap(e: &Env, soroswap: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, SOROSWAP_KEY), soroswap);
}
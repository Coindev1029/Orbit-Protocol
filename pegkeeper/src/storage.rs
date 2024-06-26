use soroban_sdk::{Address, Env, unwrap::UnwrapOptimized, contracttype};

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD_INSTANCE: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP_INSTANCE: u32 = LEDGER_THRESHOLD_INSTANCE + ONE_DAY_LEDGERS; // ~ 31 days

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ADMIN,
    TREASURY(Address), // mapping token address to treasury addres
    BALANCE,
    MAXIMUMDURATION,
    RECEIVER,
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

/// Fetch the current balance
///
pub fn _get_balance(e: &Env) -> i128 {
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

/// Fetch the maximum duration for swap transaction
///
pub fn get_maximum_duration(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::MAXIMUMDURATION)
        .unwrap_optimized()
}

/// Set the maximum duration for swap transaction
///
/// ### Arguments
/// * `maximum_duration` - The maximum_duration for swap transaction
pub fn set_maximum_duration(e: &Env, maximum_duration: &u64) {
    e.storage()
        .instance()
        .set(&DataKey::MAXIMUMDURATION, maximum_duration);
}

/// Fetch the address for receiver contract
///
pub fn get_receiver(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::RECEIVER)
        .unwrap_optimized()
}

/// Set the address for receiver contract
///
/// ### Arguments
/// * `receiver` - The address for receiver contract
pub fn set_receiver(e: &Env, receiver: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::RECEIVER, receiver);
}

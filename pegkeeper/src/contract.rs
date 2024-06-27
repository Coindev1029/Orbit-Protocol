use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, Address, Env};
use crate::{errors::PegkeeperError, storage};
#[contract]
pub struct PegkeeperContract;

#[contractclient(name="PegkeeperClient")]
pub trait Pegkeeper {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address);

    /// Execute operation
    ///
    /// ### Arguments
    /// * `caller` - The Address for the caller
    /// * `token` - The Address for the token
    /// * `amount` - The Amount for the flashloan
    /// * `fee` - The Fee for the flashloan
    fn exe_op(e: Env, caller: Address, token: Address, blend_pool: Address, liquidation: Address, amount: i128);
}

#[contractimpl]
impl Pegkeeper for PegkeeperContract {
    fn initialize(e: Env, admin: Address) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, PegkeeperError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
    }
    fn exe_op(e: Env, caller: Address, token: Address, blend_pool: Address, liquidation: Address, amount: i128) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        log!(&e, "================================= Real: Pegkeeper Function Start ================================");
        let token_client = token::Client::new(
            &e,
            &token
        );

        // Perform liquidation & swap on blend & soroswap
        // ...
        
        token_client.approve(
            &e.current_contract_address(),
            &caller,
            &amount,
            &(e.ledger().sequence() + 1),
        );
        log!(&e, "================================= Real: Pegkeeper Function End ================================");
    }
}


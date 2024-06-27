use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, Address, Env};
use crate::{errors::MockPegkeeperError, storage};
#[contract]
pub struct MockPegkeeperContract;

#[contractclient(name="MockPegkeeperClient")]
pub trait MockPegkeeper {
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
    fn exe_op(e: Env, caller: Address, token: Address, amount: i128, fee: i128);
}

#[contractimpl]
impl MockPegkeeper for MockPegkeeperContract {
    fn initialize(e: Env, admin: Address) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, MockPegkeeperError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
    }
    fn exe_op(e: Env, caller: Address, token: Address, amount: i128, fee: i128) {
        caller.require_auth();

        log!(&e, "================================= Pegkeeper Function Start ================================");
        let token_client = token::Client::new(
            &e,
            &token
        );

        // Perform liquidation & swap on blend & soroswap
        // ...
        
        let total_amount = amount + fee;
        
        token_client.approve(
            &e.current_contract_address(),
            &caller,
            &total_amount,
            &(e.ledger().sequence() + 1),
        );
        log!(&e, "================================= Pegkeeper Function End ================================");
    }
}


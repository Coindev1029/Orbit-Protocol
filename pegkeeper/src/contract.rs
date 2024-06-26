use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, symbol_short, vec, Address, Env, IntoVal, Val, Vec};
use crate::{
    balances, 
    errors::PegkeeperError, storage
};

#[contract]
pub struct PegkeeperContract;

#[contractclient(name="PegkeeperClient")]
pub trait Pegkeeper {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `receiver` - The Address for receiver contract
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address, receiver: Address, maximum_duration: u64);

    /// (Admin only) Set a new address as the admin of this pool
    ///
    /// ### Arguments
    /// * `new_admin` - The new admin address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn set_admin(e: Env, admin: Address);

    /// (Admin only) Add a new treasury for the flashloan
    ///
    /// ### Arguments
    /// * `new_token_address` - The new token address
    /// * `new_treasury` - The new pegkeeper address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn add_treasury(e: Env, new_token_address: Address, new_treasury: Address);

    /// (Admin only) Set the maximum duration for swap transaction
    ///
    /// ### Arguments
    /// * `maximum_duration` - The new maximum_duration for swap transaction
    /// ### Panics
    /// If the caller is not the admin
    fn set_maximum_duration(e: Env, maximum_duration: u64);

    /// Flash loan specific amount from specific treasury by using token address
    /// ### Arguments
    /// * `token_address` - The token address for flash loan
    /// * `amount` - The amount to flash loan
    ///
    /// ### Panics
    /// If there is no profit
    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), PegkeeperError>;
    
    /// Get token address
    fn get_treasury(e: Env, token_address: Address) -> Address;

    /// Get maximum duration for the swap transaction
    fn get_maximum_duration(e: Env) -> u64;

}

#[contractimpl]
impl Pegkeeper for PegkeeperContract {
    fn initialize(e: Env, admin: Address, receiver: Address, maximum_duration: u64) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, PegkeeperError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
        storage::set_balance(&e, &0);
        storage::set_receiver(&e, &receiver);
        storage::set_maximum_duration(&e, &maximum_duration);
    }

    fn set_admin(e: Env, admin: Address) {
        storage::extend_instance(&e);
        let old_admin = storage::get_admin(&e);
        old_admin.require_auth();

        storage::set_admin(&e, &admin);
    }

    fn add_treasury(e: Env, new_token_address: Address, new_treasury: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        storage::set_treasury(&e, new_token_address, &new_treasury);
    }

    fn set_maximum_duration(e: Env, maximum_duration: u64) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        storage::set_maximum_duration(&e, &maximum_duration);
    }

    fn get_treasury(e: Env, token_address: Address) -> Address {
        storage::extend_instance(&e);
        storage::get_treasury(&e, token_address)
    }

    fn get_maximum_duration(e: Env) -> u64 {
        storage::extend_instance(&e);
        storage::get_maximum_duration(&e)
    }

    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&e);

        let treasury_address = storage::get_treasury(&e, token_address.clone());
        let receiver_address = storage::get_receiver(&e);

        let balance = balances::get_balance(&e, token_address);
        storage::set_balance(&e, &balance);
        
        let mut init_args: Vec<Val> = vec![&e];
        init_args.push_back(receiver_address.into_val(&e));
        init_args.push_back(amount.into_val(&e));
        e.invoke_contract::<Val>(&treasury_address, &symbol_short!("fl_loan"), init_args);

        log!(&e, "================================= Real: Pegkeeper FlashLoan End ================================");

        Ok(())
    }
    
}


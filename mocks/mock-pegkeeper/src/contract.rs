#![no_std]
use core::array;

use soroban_sdk::{contract, contractclient, contractimpl, panic_with_error, token, vec, Address, Env, Symbol};
use crate::{
    balances, dependencies::{
        blend::{Client as BlendClient, Positions, Request}, 
        router::Client as SoroswapRouter, 
        treasury::Client as TreasuryClient
    }, 
    errors::MockPegkeeperError, storage
};



#[contract]
pub struct MockPegkeeperContract;

#[contractclient(name="MockPegkeeperClient")]
pub trait MockPegkeeper {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address, maximum_duration: u64);

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
    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), MockPegkeeperError>;

    /// ### Arguments
    /// * `token_address` - The token address for flash loan
    /// * `tresury_address` - The treasury address for flash loan
    /// * `amount` - The amount to flash loan
    /// * `treasury_fee` - The fee of treasury
    ///
    /// ### Panics
    /// If there is no profit
    fn flashloan_receive(e: Env, treasury_address: Address, amount: i128) -> Result<(), MockPegkeeperError>;
    
    /// Get token address
    fn get_treasury(e: Env, token_address: Address) -> Address;

    /// Get maximum duration for the swap transaction
    fn get_maximum_duration(e: Env) -> u64;

}

#[contractimpl]
impl MockPegkeeper for MockPegkeeperContract {
    fn initialize(e: Env, admin: Address, maximum_duration: u64) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, MockPegkeeperError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
        storage::set_balance(&e, &0);
        storage::set_maximum_duration(&e, &maximum_duration);
    }

    fn set_admin(e: Env, admin: Address) {
        storage::extend_instance(&e);

        storage::set_admin(&e, &admin);
    }

    fn add_treasury(e: Env, new_token_address: Address, new_treasury: Address) {
        storage::extend_instance(&e);

        storage::set_treasury(&e, new_token_address, &new_treasury);
    }

    fn set_maximum_duration(e: Env, maximum_duration: u64) {
        storage::extend_instance(&e);

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

    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), MockPegkeeperError> {
        storage::extend_instance(&e);

        e.events().publish((Symbol::new(&e, "start_flash_loan"), token_address.clone(), amount.clone()), ("Success"));
        let treasury_address = storage::get_treasury(&e, token_address.clone());
        
        let treasury_client = TreasuryClient::new(&e, &treasury_address);
        // treasury_client.flash_loan(&amount);

        e.events().publish((Symbol::new(&e, "call_flash_loan"), token_address.clone(), amount.clone()), ("Success"));
        Ok(())
    }
    fn flashloan_receive(e: Env, treasury_address: Address, amount: i128) -> Result<(), MockPegkeeperError> {
        storage::extend_instance(&e);
    
        treasury_address.require_auth();
        
        e.events().publish((Symbol::new(&e, "flash_loan_receive"), treasury_address.clone(), amount.clone()), ("Success"));

        Ok(())
    }
}


#![no_std]
use soroban_sdk::{contract, contractclient, contractimpl, panic_with_error, token, Address, Env, vec};
use crate::{
    balances, dependencies::{
        blend::{Client as BlendClient, Positions, Request}, 
        router::Client as SoroswapRouter, 
        treasury::Client as TreasuryClient
    }, 
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
    /// * `blend` - The Address for the blend pool
    /// * `soroswap` - The Address for the soroswap
    ///
    fn initialize(e: Env, admin: Address, blend: Address, soroswap: Address);

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

    /// Flash loan specific amount from specific treasury by using token address
    /// ### Arguments
    /// * `token_address` - The token address for flash loan
    /// * `amount` - The amount to flash loan
    ///
    /// ### Panics
    /// If there is no profit
    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), PegkeeperError>;

    /// ### Arguments
    /// * `token_address` - The token address for flash loan
    /// * `tresury_address` - The treasury address for flash loan
    /// * `amount` - The amount to flash loan
    /// * `treasury_fee` - The fee of treasury
    ///
    /// ### Panics
    /// If there is no profit
    fn flashloan_receive(e: Env, token_address: Address, treasury_address: Address, blend_address: Address, soroswap_address: Address, amount: i128, treasury_fee: i128) -> Result<(), PegkeeperError>;
    /// Get token address
    fn get_treasury(e: Env, token_address: Address) -> Address;

}

#[contractimpl]
impl Pegkeeper for PegkeeperContract {
    fn initialize(e: Env, admin: Address, blend: Address, soroswap: Address) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, PegkeeperError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
    }

    fn add_treasury(e: Env, new_token_address: Address, new_treasury: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        storage::set_treasury(&e, new_token_address, &new_treasury);
    }

    fn set_admin(e: Env, admin: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        storage::set_admin(&e, &admin);
    }

    fn get_treasury(e: Env, token_address: Address) -> Address {
        storage::get_treasury(&e, token_address)
    }

    fn flash_loan(e: Env, token_address: Address, amount: i128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&e);

        let treasury_address = storage::get_treasury(&e, token_address.clone());
        
        let balance = balances::get_balance(&e, token_address);
        storage::set_balance(&e, &balance);
        /// invoke treasury flash_loan func
        let treasury_client = TreasuryClient::new(&e, &treasury_address);
        treasury_client.flash_loan(&amount);
        Ok(())
    }
    fn flashloan_receive(e: Env, token_address: Address, treasury_address: Address, blend_address: Address, soroswap_address: Address, amount: i128, treasury_fee: i128) -> Result<(), PegkeeperError> {
        storage::extend_instance(&e);
    
        treasury_address.require_auth();
        
        // Check balance of token of contract
        let balance_after = balances::get_balance(&e, token_address.clone());
        let balance_before =storage::get_balance(&e);

        if balance_after - balance_before < amount {
            return Err(PegkeeperError::InsufficientBalance);
        }
    
        // Interact with blend
        let blend_client = BlendClient::new(&e, &blend_address);
        let positions = blend_client.submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &vec![
            &e,
            Request {
                request_type: 6_u32, // FillUserLiquidationAuction RequestType
                address: token_address.clone(),
                amount,
            }]);
        
        // Trades on any other protocols
        // let soroswap_router = SoroswapRouter::new(&e, &soroswap_address);

        // Repay the flash loan amount + treasury fee to treasury
        balances::transfer_amount(&e, token_address, treasury_address, amount + treasury_fee);

        Ok(())
    }
    
}


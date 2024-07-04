use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, Address, Env, vec, Val, Vec, IntoVal, Symbol};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
use crate::{errors::PegkeeperError, storage, helper};
use crate::dependencies::{
    router::{Client as RouterClient},
    pool::{Client as PoolClient, Request},
};
#[contract]
pub struct PegkeeperContract;

#[contractclient(name="PegkeeperClient")]
pub trait Pegkeeper {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address, router: Address);

    /// Execute operation
    ///
    /// ### Arguments
    /// * `pair` - Soroswap address
    /// * `auction_creator` - Auction creator address
    /// * `token_a` - Token A address
    /// * `token_a_bid_amount` - Token A bid amount
    /// * `token_b` - Token B address
    /// * `token_b_lot_amount` - Token B lot amount
    /// * `blend_pool` - Blend pool address
    /// * `liq_amount` - Liquidate amount
    fn fl_receive(e: Env, pair: Address, auction_creator: Address, token_a: Address, token_a_bid_amount: i128, token_b: Address, token_b_lot_amount: i128, blend_pool: Address, liq_amount: i128);
}

#[contractimpl]
impl Pegkeeper for PegkeeperContract {
    fn initialize(e: Env, admin: Address, router: Address) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, PegkeeperError::AlreadyInitializedError);
        }

        storage::set_router(&e, &router);
        storage::set_admin(&e, &admin);
    }
    fn fl_receive(e: Env, pair: Address, auction_creator: Address, token_a: Address, token_a_bid_amount: i128, token_b: Address, token_b_lot_amount: i128, blend_pool: Address, liq_amount: i128) {
        log!(&e, "================================= Real: Pegkeeper Function Start ================================");
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();
        log!(&e, "================================= Real: Pegkeeper Function Passed Auth ================================");
        let token_client = token::Client::new(&e, &token_a);
        let balance_before = token_client.balance(&e.current_contract_address());

        log!(&e, "================================= Real: Pegkeeper Function Prepare for liquidation ================================");
        helper::liquidate(&e, auction_creator, token_a.clone(), token_a_bid_amount.clone(), token_b.clone(), token_b_lot_amount.clone(), blend_pool.clone(), liq_amount.clone());
        log!(&e, "================================= Real: Pegkeeper Function Passed Liquidation ================================");
        helper::swap(&e, pair, token_b.clone(), token_a.clone(), token_b_lot_amount.clone(), 0);
        log!(&e, "================================= Real: Pegkeeper Function Passed Swap ================================");

        let balance_after = token_client.balance(&e.current_contract_address());
        let profit = balance_after - balance_before;
        
        // send this profit to speicfic address or caller

        log!(&e, "================================= Real: Profit {} {} {} ================================", profit, balance_after, balance_before);
        //transfer profit to caller
        // token_client.transfer(&e.current_contract_address(), &caller, &profit);

        token_client.approve(
            &e.current_contract_address(),
            &admin,
            &token_a_bid_amount,
            &(e.ledger().sequence() + 1),
        );

        log!(&e, "================================= Real: Pegkeeper Function End ================================");
    }
}

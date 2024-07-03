use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, vec, Vec, Address, Env, IntoVal, Symbol, Val};
use crate::dependencies::pool::{Client as PoolClient, Request};
use crate::dependencies::router::{Client as RouterClient};
use crate::helper;
use crate::{errors::MockPegkeeperError, storage};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
#[contract]
pub struct MockPegkeeperContract;

#[contractclient(name="MockPegkeeperClient")]
pub trait MockPegkeeper {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address, router: Address);

    fn fl_receive(e: Env, pair: Address, auction_creator: Address, token_a: Address, token_a_bid_amount: i128, token_b: Address, token_b_lot_amount: i128, blend_pool: Address, liq_amount: i128);
}

#[contractimpl]
impl MockPegkeeper for MockPegkeeperContract {
    fn initialize(e: Env, admin: Address, router: Address) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, MockPegkeeperError::AlreadyInitializedError);
        }

        storage::set_router(&e, &router);
        storage::set_admin(&e, &admin);
    }
    fn fl_receive(e: Env, pair: Address, auction_creator: Address, token_a: Address, token_a_bid_amount: i128, token_b: Address, token_b_lot_amount: i128, blend_pool: Address, liq_amount: i128) {
        storage::extend_instance(&e);

        // let admin = storage::get_admin(&e);
        // admin.require_auth();
        helper::liquidate(&e, auction_creator, token_a.clone(), token_a_bid_amount.clone(), token_b.clone(), token_b_lot_amount.clone(), blend_pool.clone(), liq_amount.clone());
        helper::swap(&e, pair, token_b.clone(), token_a.clone(), token_b_lot_amount.clone(), 0);

        
        log!(&e, "================================= MockPegkeeper  fl_receive function End ================================");
    }
}


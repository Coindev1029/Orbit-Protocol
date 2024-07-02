use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, vec, Vec, Address, Env, IntoVal, Symbol, Val};
use crate::dependencies::pool::{Client as PoolClient, Request};
use crate::dependencies::router::{Client as RouterClient};
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

    /// Execute operation
    ///
    /// ### Arguments
    /// * `caller` - The Address for the caller
    /// * `token` - The Address for the token
    /// * `amount` - The Amount for the flashloan
    /// * `fee` - The Fee for the flashloan
    fn fl_receive(e: Env, caller: Address, token: Address, blend_pool: Address, liquidation: Address, amount: i128);

    /// Execute operation
    ///
    /// ### Arguments
    /// * `caller` - The Address for the caller
    /// * `token` - The Address for the token
    /// * `amount` - The Amount for the flashloan
    /// * `fee` - The Fee for the flashloan
    fn liquidate(e: Env, auction_creator: Address, ousd: Address, ousd_bid_amount: i128, xlm: Address, xlm_lot_amount: i128, blend_pool: Address, amount: i128);

    fn swap(e: Env, pair: Address, token_a: Address, token_b: Address, amount_a: i128, amount_b: i128);
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
    fn fl_receive(e: Env, caller: Address, token: Address, blend_pool: Address, liquidation: Address, amount: i128) {
        storage::extend_instance(&e);

        let admin = storage::get_admin(&e);
        admin.require_auth();

        log!(&e, "================================= MockPegkeeper  fl_receive function End ================================");
    }

    fn liquidate(e: Env, auction_creator: Address, ousd: Address, ousd_bid_amount: i128, xlm: Address, xlm_lot_amount: i128, blend_pool: Address, amount: i128) {
        log!(&e, "================================= MockPegkeeper  liquidation Function ================================");
        storage::extend_instance(&e);

        let fill_requests = vec![
            &e,
            Request {
                request_type: 6 as u32,
                address: auction_creator.clone(), // liquidationAuction
                amount: amount.clone(),
            },
            Request {
                request_type: 5 as u32, // Repay
                address: ousd.clone(),
                amount: ousd_bid_amount,
            },
            Request {
                request_type: 3 as u32, // Withdraw
                address: xlm.clone(),
                amount: xlm_lot_amount,
            },
        ];

        let args: Vec<Val> = vec![
            &e,
            e.current_contract_address().into_val(&e),
            blend_pool.into_val(&e),
            ousd_bid_amount.into_val(&e),
        ];
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: ousd.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: args.clone(),
                },
                sub_invocations: vec![&e],
            })
        ]);

        log!(&e, "================================= MockPegkeeper  Fill Request ================================");
        PoolClient::new(&e, &blend_pool).submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &fill_requests);

        log!(&e, "================================= MockPegkeeper  liquidation End ================================");
    }

    fn swap(e: Env, pair: Address, token_a: Address, token_b: Address, amount_a: i128, amount_b: i128) {
        log!(&e, "================================= MockPegkeeper  Swap Function ================================");
        storage::extend_instance(&e);

        let router = storage::get_router(&e);
        let router_client = RouterClient::new(&e, &router);

        let path = vec![
            &e,
            token_a.clone(),
            token_b.clone(),
        ];
        let args: Vec<Val> = vec![
            &e,
            e.current_contract_address().into_val(&e),
            pair.into_val(&e),
            amount_a.into_val(&e),
        ];
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_a.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: args.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);
        router_client.swap_exact_tokens_for_tokens(&amount_a, &amount_b, &path, &e.current_contract_address(), &u64::MAX);
        log!(&e, "================================= MockPegkeeper  Swap End ================================");
    }
}


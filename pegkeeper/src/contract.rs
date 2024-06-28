use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, token, Address, Env, vec, Val, Vec, IntoVal, Symbol};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
use crate::{errors::PegkeeperError, storage};
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
    /// * `caller` - The Address for the caller
    /// * `token` - The Address for the token
    /// * `amount` - The Amount for the flashloan
    /// * `fee` - The Fee for the flashloan
    fn exe_op(e: Env, caller: Address, token: Address, collateral: Address, pair: Address, blend_pool: Address, liquidation: Address, amount_in: i128, amount_out: i128, percent: i128);
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
    fn exe_op(e: Env, caller: Address, token: Address, collateral: Address, pair: Address, blend_pool: Address, liquidation: Address, amount_in: i128, amount_out: i128, percent: i128) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        let token_client = token::Client::new(&e, &token);
        let balance_before = token_client.balance(&e.current_contract_address());

        let liquidation_request = vec![
            &e,
            Request {
                request_type: 6 as u32, // liquidationAuction
                address: liquidation.clone(),
                amount: percent.clone(),
            },
            Request {
                request_type: 5 as u32, // Repay
                address: token.clone(),
                amount: amount_in.clone(),
            },
            Request {
                request_type: 3 as u32, // Withdraw
                address: collateral.clone(),
                amount: amount_out.clone(),
            },
        ];

        let args: Vec<Val> = vec![
            &e,
            e.current_contract_address().into_val(&e),
            blend_pool.into_val(&e),
            amount_in.into_val(&e),
        ];
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: token.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: args.clone(),
                },
                sub_invocations: vec![&e],
            })
        ]);
        PoolClient::new(&e, &blend_pool).submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &liquidation_request);

        let router = storage::get_router(&e);
        let router_client = RouterClient::new(&e, &router);

        let path = vec![
            &e,
            collateral.clone(),
            token.clone(),
        ];
        let args: Vec<Val> = vec![
            &e,
            e.current_contract_address().into_val(&e),
            pair.into_val(&e),
            amount_out.into_val(&e),
        ];
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: collateral.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: args.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);
        router_client.swap_exact_tokens_for_tokens(&amount_out, &amount_in, &path, &e.current_contract_address(), &u64::MAX);

        let balance_after = token_client.balance(&e.current_contract_address());
        let profit = balance_after - balance_before - amount_in;

        //transfer profit to caller
        token_client.transfer(&e.current_contract_address(), &caller, &profit);

        token_client.transfer(&e.current_contract_address(), &admin, &amount_in);

        log!(&e, "================================= Real: Pegkeeper Function End ================================");
    }
}


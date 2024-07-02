use sep_40_oracle::Asset;
use crate::storage;
use crate::dependencies::pool::{Client as PoolClient, Request};
use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, symbol_short, token, vec, Address, Env, IntoVal, Symbol, Val, Vec};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
use crate::errors::MockTreasuryError;
use token::Client as TokenClient;
use sep_41_token::StellarAssetClient;
use token::StellarAssetClient as TokenAdminClient;

const FLASH_LOAN: Symbol = symbol_short!("FLASHLOAN");

#[contract]
pub struct MockTreasuryContract;

#[contractclient(name="MockTreasuryClient")]
pub trait MockTreasury {

    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `token` - The Address for the token
    /// * `blend_pool` - The Address for the blend pool
    ///
    fn initialize(e: Env, admin: Address, bridge_oracle: Address, pegkeeper: Address);


    fn deploy_stablecoin(e: Env, token: Address, asset: Asset, blend_pool: Address);

    /// (Admin only) Set a new address as the admin of this pool
    ///
    /// ### Arguments
    /// * `new_admin` - The new admin address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn set_admin(e: Env, admin: Address);

    /// Flashloan function for keeping the peg of stablecoins
    ///
    /// ### Arguments
    /// * `caller` - The Address of the caller
    /// * `token` - The Address of the token
    /// * `liquidation` - The Address of the liquidation contract
    /// * `amount` - The amount of the flashloan
    fn keep_peg(e: Env, caller: Address, token: Address, liquidation: Address, amount: i128);

    /// (Admin only) Increase the supply of the pool
    ///
    /// ### Arguments
    /// * `amount` - The amount to increase the supply by
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn increase_supply(e: Env, token: Address, amount: i128);
}

#[contractimpl]
impl MockTreasury for MockTreasuryContract {

    fn initialize(e: Env, admin: Address, bridge_oracle: Address, pegkeeper: Address) {
        storage::extend_instance(&e);
        if storage::is_init(&e) {
            panic_with_error!(&e, MockTreasuryError::AlreadyInitializedError);
        }

        storage::set_pegkeeper(&e, &pegkeeper);
        storage::set_bridge_oracle(&e, &bridge_oracle);
        storage::set_admin(&e, &admin);
    }

    fn deploy_stablecoin(e: Env, token: Address, asset: Asset, blend_pool: Address) {
        storage::extend_instance(&e);

        let admin = storage::get_admin(&e);
        admin.require_auth();

        let bridge_oracle = storage::get_bridge_oracle(&e);
        let token_asset = Asset::Stellar(token.clone());
        let add_asset_args = vec![
            &e,
            token_asset.into_val(&e),
            asset.into_val(&e),
        ];
        e.invoke_contract::<Val>(&bridge_oracle, &Symbol::new(&e, "add_asset"), add_asset_args);


        storage::set_blend_pool(&e, &token, &blend_pool);
    }

    fn set_admin(e: Env, new_admin: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();
        new_admin.require_auth();

        storage::set_admin(&e, &new_admin);
    }

    fn increase_supply(e: Env, token: Address, amount: i128) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        let blend = storage::get_blend_pool(&e, &token);
        StellarAssetClient::new(&e, &token).mint(&e.current_contract_address(), &amount);
        let args: Vec<Val> = vec![
            &e,
            e.current_contract_address().into_val(&e),
            blend.into_val(&e),
            amount.into_val(&e),
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
        PoolClient::new(&e, &blend).submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &vec![
            &e,
            Request {
                request_type: 0_u32, // SUPPLY RequestType
                address: token.clone(),
                amount,
            },
        ]);
    }

    fn keep_peg(e: Env, caller: Address, token: Address, liquidation: Address, amount: i128) {
        storage::extend_instance(&e);
        
        log!(&e, "================================= Mock: Treasury FlashLoan Function Start ============================");

        let pegkeeper: Address = storage::get_pegkeeper(&e);
        let blend_pool: Address = storage::get_blend_pool(&e, &token);

        StellarAssetClient::new(&e, &token).mint(&pegkeeper, &amount);

        // Execute operation
        let fl_receive_args = vec![
            &e,
            caller.into_val(&e),
            token.into_val(&e),
            blend_pool.into_val(&e),
            liquidation.into_val(&e),
            amount.into_val(&e),
        ];
        e.invoke_contract::<Val>(&pegkeeper, &Symbol::new(&e, "fl_receive"), fl_receive_args);
        log!(&e, "================================= Mock: Treasury FlashLoan Function End ============================");
    }
}

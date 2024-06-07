use crate::{
    errors::TreasuryFactoryError,
    storage::{self, TreasuryInitMeta},
};

use soroban_sdk::{contract, contractclient, contractimpl, panic_with_error, vec, Address, BytesN, Env, IntoVal, Symbol, Val, Vec};
use sep_40_oracle::Asset;


#[contract]
pub struct TreasuryFactoryContract;

#[contractclient(name = "TreasuryFactoryClient")]
pub trait TreasuryFactory {

    /// Initializes the treasury factory
    ///
    /// # Arguments
    /// * `admin` - The admin address
    /// * `treasury_init_meta` - The treasury init meta
    fn initialize(e: Env, admin: Address, bridge_oracle: Address, treasury_init_meta: TreasuryInitMeta);

    /// (Admin only) Deploys a new treasury with token and pool
    ///
    /// # Arguments
    /// * `salt` - The salt for the deployment
    /// * `token_address` - The token address
    /// * `blend_pool` - The blend pool address
    fn deploy(e: Env, salt: BytesN<32>, token_address: Address, asset: Asset, blend_pool: Address) -> Address;

    /// (Admin only) Set a new address as the admin of this pool
    ///
    /// ### Arguments
    /// * `new_admin` - The new admin address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn set_admin(e: Env, admin: Address);

    /// Fetch the current bridge oracle Address
    fn get_oracle(e: Env) -> Address;

    /// (Admin only) Set a new oracle for the bridge oracle
    ///
    /// ### Arguments
    /// * `oracle` - The new oracle address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn set_oracle(e: Env, oracle: Address);

    /// Checks if contract address was deployed by the factory
    ///
    /// Returns true if treasury was deployed by factory and false otherwise
    ///
    /// # Arguments
    /// * `treasury_id` - The contract address to be checked
    fn is_treasury(e: Env, treasury_id: Address) -> bool;
}

#[contractimpl]
impl TreasuryFactory for TreasuryFactoryContract {
    fn initialize(e: Env, admin: Address, bridge_oracle: Address, treasury_init_meta: TreasuryInitMeta) {
        storage::extend_instance(&e);
        if storage::is_init(&e) {
            panic_with_error!(&e, TreasuryFactoryError::AlreadyInitializedError);
        }

        storage::set_bridge_oracle(&e, &bridge_oracle);
        storage::set_pool_init_meta(&e, &treasury_init_meta);
        storage::set_admin(&e, &admin);

        e.events().publish(("TreasuryFactory", Symbol::new(&e, "init")), (admin.clone(), bridge_oracle.clone(), treasury_init_meta.clone()));
    }

    fn deploy(e: Env, salt: BytesN<32>, token_address: Address, asset: Asset, blend_pool: Address) -> Address {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        // Add the asset to the bridge oracle
        let bridge_oracle = storage::get_bridge_oracle(&e);
        let token_asset = Asset::Stellar(token_address.clone());
        let bridge_oracle_args: Vec<Val> = vec![
            &e,
            token_asset.into_val(&e),
            asset.into_val(&e),
        ];
        e.invoke_contract::<Val>(&bridge_oracle, &Symbol::new(&e, "add_asset"), bridge_oracle_args);

        // Deploy the treasury
        let treasury_init_meta = storage::get_pool_init_meta(&e);
        let treasury_hash = treasury_init_meta.treasury_hash;
        let treasury_id = e.deployer().with_current_contract(salt).deploy(treasury_hash);
        let treasury_init_args = vec![
            &e,
            admin.into_val(&e),
            token_address.into_val(&e),
            blend_pool.into_val(&e),
        ];
        e.invoke_contract::<Val>(&treasury_id, &Symbol::new(&e, "initialize"), treasury_init_args);
        
        storage::set_deployed(&e, &token_address, &treasury_id);

        e.events().publish(("TreasuryFactory", Symbol::new(&e, "deploy")), treasury_id.clone());
        treasury_id
    }

    fn set_admin(e: Env, new_admin: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();
        new_admin.require_auth();

        storage::set_admin(&e, &new_admin);

        e.events().publish(("TreasuryFactory", Symbol::new(&e, "set_admin")), new_admin.clone());
    }

    fn get_oracle(e: Env) -> Address {
        storage::extend_instance(&e);
        storage::get_bridge_oracle(&e)
    }

    fn set_oracle(e: Env, oracle: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();

        let bridge_oracle = storage::get_bridge_oracle(&e);
        let args = vec![&e, oracle.into_val(&e)];
        e.invoke_contract::<Val>(&bridge_oracle, &Symbol::new(&e, "set_oracle"), args);
    }

    fn is_treasury(e: Env, treasury_id: Address) -> bool {
        storage::extend_instance(&e);
        storage::is_deployed(&e, &treasury_id)
    }
}
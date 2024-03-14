use crate::{
    errors::TreasuryFactoryError,
    storage::{self, TreasuryInitMeta},
};

use soroban_sdk::{contract, contractclient, contractimpl, panic_with_error, vec, Address, BytesN, Env, IntoVal, Symbol, Val};

#[contract]
pub struct TreasuryFactoryContract;

#[contractclient(name = "TreasuryFactoryClient")]
pub trait TreasuryFactory {

    /// Initializes the treasury factory
    ///
    /// # Arguments
    /// * `admin` - The admin address
    /// * `treasury_init_meta` - The treasury init meta
    fn initialize(e: Env, admin: Address, treasury_init_meta: TreasuryInitMeta);

    /// (Admin only) Deploys a new treasury with token and pool
    ///
    /// # Arguments
    /// * `salt` - The salt for the deployment
    /// * `token_name` - The name of the token
    /// * `token_symbol` - The symbol of the token
    /// * `token_address` - The asset of the oracle to use for the pool
    /// * `oracle` - The address of the oracle to use for the pool
    /// * `pool_name` - The name of the pool
    /// * `backstop_take_rate` - The backstop take rate of the pool
    /// * `max_positions` - The maximum number of positions the pool
    fn deploy(e: Env, salt: BytesN<32>, token_address: Address, blend_pool: Address) -> Address;

    /// (Admin only) Set a new address as the admin of this pool
    ///
    /// ### Arguments
    /// * `new_admin` - The new admin address
    ///
    /// ### Panics
    /// If the caller is not the admin
    fn set_admin(e: Env, admin: Address);

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
    fn initialize(e: Env, admin: Address, treasury_init_meta: TreasuryInitMeta) {
        storage::extend_instance(&e);
        if storage::get_is_init(&e) {
            panic_with_error!(&e, TreasuryFactoryError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
        storage::set_pool_init_meta(&e, &treasury_init_meta);
        storage::set_is_init(&e);
    }

    fn deploy(e: Env, salt: BytesN<32>, token_address: Address, blend_pool: Address) -> Address {
        storage::extend_instance(&e);
        if !storage::get_is_init(&e) {
            panic_with_error!(&e, TreasuryFactoryError::InternalError);
        }

        let admin = storage::get_admin(&e);
        admin.require_auth();

        let treasury_init_meta = storage::get_pool_init_meta(&e);
        let treasury_hash = treasury_init_meta.treasury_hash;
        let treasury_id = e.deployer().with_current_contract(salt).deploy(treasury_hash);
        
        // Check if it is a blend pool
        // let pool_factory = treasury_init_meta.pool_factory;
        // let pool_args = vec![
        //     &e,
        //     blend_pool.into_val(&e),
        // ];
        // let pool = e.invoke_contract::<bool>(&pool_factory, &Symbol::new(&e, "is_pool"), pool_args);
        // if !pool {
        //     panic_with_error!(&e, TreasuryFactoryError::InternalError);
        // }

        
        // Init the treasury
        let treasury_init_args = vec![
            &e,
            admin.into_val(&e),
            token_address.into_val(&e),
            blend_pool.into_val(&e),
        ];
        e.invoke_contract::<Val>(&treasury_id, &Symbol::new(&e, "initialize"), treasury_init_args);
        
        storage::set_deployed(&e, &treasury_id);
        treasury_id
    }

    fn set_admin(e: Env, new_admin: Address) {
        storage::extend_instance(&e);
        let admin = storage::get_admin(&e);
        admin.require_auth();
        new_admin.require_auth();

        storage::set_admin(&e, &new_admin);
    }

    fn is_treasury(e: Env, treasury_id: Address) -> bool {
        storage::extend_instance(&e);
        storage::is_deployed(&e, &treasury_id)
    }
}
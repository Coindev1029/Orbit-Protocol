use soroban_sdk::{testutils::Address as _, Address, Env};

mod pool_factory_contract {
    soroban_sdk::contractimport!(
        file = "../wasm/pool_factory.wasm"
    );
}
pub use pool_factory_contract::{Client as PoolFactoryClient, WASM as POOL_FACTORY_WASM, PoolInitMeta};

pub fn create_pool_factory<'a>(e: &Env) -> (Address, PoolFactoryClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, POOL_FACTORY_WASM);
    (contract_id.clone(), PoolFactoryClient::new(e, &contract_id))
}

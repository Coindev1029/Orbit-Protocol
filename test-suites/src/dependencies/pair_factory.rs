use soroban_sdk::{testutils::Address as _, Address, Env};

mod pair_factory_contract {
    soroban_sdk::contractimport!(file = "../wasm/pair_factory.wasm");
}
pub use pair_factory_contract::{Client as PairFactoryClient, WASM as PAIR_FACTORY_WASM};

pub fn create_pair_factory<'a>(e: &Env) -> (Address, PairFactoryClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, PAIR_FACTORY_WASM);
    (contract_id.clone(), PairFactoryClient::new(e, &contract_id))
}

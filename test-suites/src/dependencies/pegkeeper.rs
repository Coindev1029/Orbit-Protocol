use soroban_sdk::{testutils::Address as _, Address, Env};

mod pegkeeper_contract {
    soroban_sdk::contractimport!(file = "../wasm/pegkeeper.wasm");
}
pub use pegkeeper_contract::{Client as PegkeeperClient, WASM as PEGKEEPER_WASM};

pub fn create_pegkeeper<'a>(e: &Env) -> (Address, PegkeeperClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, PEGKEEPER_WASM);
    (contract_id.clone(), PegkeeperClient::new(e, &contract_id))
}

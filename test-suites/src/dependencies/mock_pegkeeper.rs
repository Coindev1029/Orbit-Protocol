use soroban_sdk::{testutils::Address as _, Address, Env};

mod mock_pegkeeper_contract {
    soroban_sdk::contractimport!(file = "../wasm/orbit/mock_pegkeeper.wasm");
}

pub use mock_pegkeeper_contract::{Client as MockPegkeeperClient, WASM as MOCK_PEGKEEPER_WASM};

pub fn create_mock_pegkeeper<'a>(e: &Env) -> (Address, MockPegkeeperClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, MOCK_PEGKEEPER_WASM);
    (contract_id.clone(), MockPegkeeperClient::new(e, &contract_id))
}

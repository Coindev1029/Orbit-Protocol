use soroban_sdk::{testutils::Address as _, Address, Env};

mod mock_treasury_contract {
    soroban_sdk::contractimport!(file = "../wasm/orbit/mock_treasury.wasm");
}
pub use mock_treasury_contract::{Client as MockTreasuryClient, WASM as MOCK_TREASURY_WASM, Asset as MockAsset};

pub fn create_mock_treasury<'a>(e: &Env) -> (Address, MockTreasuryClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, MOCK_TREASURY_WASM);
    (contract_id.clone(), MockTreasuryClient::new(e, &contract_id))
}

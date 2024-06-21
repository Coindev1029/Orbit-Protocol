use soroban_sdk::{testutils::Address as _, Address, Env};

mod mock_receiver_contract {
    soroban_sdk::contractimport!(file = "../wasm/mock_receiver.wasm");
}
pub use mock_receiver_contract::{Client as MockReceiverClient, WASM as MOCK_RECEIVER_WASM};

pub fn create_mock_receiver<'a>(e: &Env) -> (Address, MockReceiverClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, MOCK_RECEIVER_WASM);
    (contract_id.clone(), MockReceiverClient::new(e, &contract_id))
}

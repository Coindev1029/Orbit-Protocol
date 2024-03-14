use soroban_sdk::{testutils::Address as _, Address, Env};

mod backstop_contract_wasm {
    soroban_sdk::contractimport!(file = "../wasm/backstop.wasm");
}
pub use backstop_contract_wasm::{Client as BackstopClient, WASM as BackstopWASM};

pub fn create_backstop<'a>(e: &Env) -> (Address, BackstopClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, BackstopWASM);
    (contract_id.clone(), BackstopClient::new(e, &contract_id))
}

use soroban_sdk::{testutils::Address as _, Address, Env};

mod emitter_contract {
    soroban_sdk::contractimport!(file = "../wasm/blend/emitter.wasm");
}
pub use emitter_contract::{Client as EmitterClient, WASM as EmitterWASM};

pub fn create_emitter<'a>(e: &Env) -> (Address, EmitterClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, EmitterWASM);
    (contract_id.clone(), EmitterClient::new(e, &contract_id))
}

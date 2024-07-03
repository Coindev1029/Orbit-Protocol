use soroban_sdk::{testutils::Address as _, Address, Env};

mod router_contract {
    soroban_sdk::contractimport!(file = "../wasm/soroswap/router.wasm");
}
pub use router_contract::{Client as RouterClient, WASM as ROUTER_WASM};

pub fn create_router<'a>(e: &Env) -> (Address, RouterClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, ROUTER_WASM);
    (contract_id.clone(), RouterClient::new(e, &contract_id))
}

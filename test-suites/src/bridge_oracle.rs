mod bridge_oracle_contract {
    soroban_sdk::contractimport!(file = "../wasm/bridge_oracle.wasm");
}

use soroban_sdk::{testutils::Address as _, Address, Env};
pub use bridge_oracle_contract::{Client as BridgeOracleClient, WASM as BRIDGE_ORACLE_WASM};

pub fn create_bridge_oracle<'a>(e: &Env) -> (Address, BridgeOracleClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, BRIDGE_ORACLE_WASM);
    (contract_id.clone(), BridgeOracleClient::new(e, &contract_id))
}
use soroban_sdk::{testutils::Address as _, Address, Env};

mod treasury_contract {
    soroban_sdk::contractimport!(
        file = "../wasm/treasury.wasm"
    );
}

pub use treasury_contract::{Client as TreasuryClient, WASM as TREASURY_WASM};

pub fn create_treasury<'a>(e: &Env) -> (Address, TreasuryClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, TREASURY_WASM);
    (contract_id.clone(), TreasuryClient::new(e, &contract_id))
}
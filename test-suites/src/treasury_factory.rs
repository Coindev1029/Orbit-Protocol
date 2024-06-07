use soroban_sdk::{testutils::Address as _, Address, Env};

mod treasury_factory_contract {
    soroban_sdk::contractimport!(file = "../wasm/treasury_factory.wasm");
}
pub use treasury_factory_contract::{Client as TreasuryFactoryClient, WASM as TREASURY_FACTORY_WASM, TreasuryInitMeta};

pub fn create_treasury_factory<'a>(e: &Env) -> (Address, TreasuryFactoryClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_contract_wasm(&contract_id, TREASURY_FACTORY_WASM);
    (contract_id.clone(), TreasuryFactoryClient::new(e, &contract_id))
}

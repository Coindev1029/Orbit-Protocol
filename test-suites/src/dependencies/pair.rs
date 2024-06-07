use soroban_sdk::{testutils::Address as _, Address, Env};

mod pair_contract {
    soroban_sdk::contractimport!(
        file = "../wasm/pair.wasm"
    );
}
pub use pair_contract::{Client as PairClient, WASM as PAIR_WASM};
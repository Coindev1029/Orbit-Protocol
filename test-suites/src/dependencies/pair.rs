mod pair_contract {
    soroban_sdk::contractimport!(
        file = "../wasm/soroswap/pair.wasm"
    );
}
pub use pair_contract::{Client as PairClient, WASM as PAIR_WASM};
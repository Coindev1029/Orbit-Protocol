use sep_40_oracle::Asset;
use soroban_sdk::{Address, Env, Symbol};

pub(crate) const LEDGER_THRESHOLD_SHARED: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP_SHARED: u32 = 241920; // ~ 14 days


const FROM_ASSET_KEY: &str = "FomAsset";
const TO_ASSET_KEY: &str = "ToAsset";
const ORACLE_KEY: &str = "Oracle";

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD_SHARED, LEDGER_BUMP_SHARED);
}


pub fn get_from_asset(env: &Env) -> Asset {
    env.storage()
        .instance()
        .get(&Symbol::new(env, FROM_ASSET_KEY))
        .unwrap()
}

pub fn set_from_asset(env: &Env, asset: &Asset) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, FROM_ASSET_KEY), asset);
}

pub fn get_to_asset(env: &Env) -> Asset {
    env.storage()
        .instance()
        .get(&Symbol::new(env, TO_ASSET_KEY))
        .unwrap()
}

pub fn set_to_asset(env: &Env, asset: &Asset) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, TO_ASSET_KEY), asset);
}

pub fn get_oracle(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&Symbol::new(env, ORACLE_KEY))
        .unwrap()
}

pub fn set_oracle(env: &Env, address: &Address) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, ORACLE_KEY), address);
}
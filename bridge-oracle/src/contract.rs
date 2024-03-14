use sep_40_oracle::{Asset, PriceData};
use soroban_sdk::{contract, contractclient, contractimpl, vec, Address, Env, Symbol, Vec, Val, IntoVal};
use crate::storage;

#[contract]
pub struct BridgeOracleContract;

#[contractclient(name = "BridgeOracleClient")]
pub trait BridgeOracle {

    /// Initializes the bridge oracle
    ///
    /// # Arguments
    /// * `admin` - The admin address
    /// * `oracle_init_meta` - The oracle init meta
    fn initialize(e: Env, from_asset: Address, to_asset: Address, oracle: Address);

    fn decimals(env: Env) -> u32;

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData>;
}

#[contractimpl]
impl BridgeOracle for BridgeOracleContract {
    fn initialize(e: Env, from_asset: Address, to_asset: Address, oracle: Address) {
        storage::extend_instance(&e);
        let from_asset = Asset::Stellar(from_asset);
        storage::set_from_asset(&e, &from_asset);
        let to_asset = Asset::Stellar(to_asset);
        storage::set_to_asset(&e, &to_asset);
        storage::set_oracle(&e, &oracle);
    }

    fn decimals(env: Env) -> u32 {
        storage::extend_instance(&env);
        let oracle = storage::get_oracle(&env);
        let args = vec![&env];
        env.invoke_contract::<u32>(&oracle, &Symbol::new(&env, "decimals"), args)
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        storage::extend_instance(&env);
        let from_asset = storage::get_from_asset(&env);
        let oracle = storage::get_oracle(&env);

        let is_same_asset = match (&from_asset, &asset) {
            (Asset::Stellar(a), Asset::Stellar(b)) => a == b,
            _ => false,
        };

        let mut args: Vec<Val> = vec![&env];
        if is_same_asset {
            args.push_back(storage::get_to_asset(&env).into_val(&env))
        } else {
            args.push_back(asset.into_val(&env));
        }
        env.invoke_contract::<Option<PriceData>>(&oracle, &Symbol::new(&env, "lastprice"), args)
    }
}
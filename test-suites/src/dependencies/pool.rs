use soroban_sdk;
mod pool_contract {
    soroban_sdk::contractimport!(file = "../wasm/blend/pool.wasm");
}

pub use pool_contract::{Client as PoolClient, Positions, PoolDataKey, ReserveConfig, ReserveData, ReserveEmissionsData, ReserveEmissionsConfig, PoolConfig,  Request, ReserveEmissionMetadata, Reserve, WASM as POOL_WASM};

#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum RequestType {
    Supply = 0,
    Withdraw = 1,
    SupplyCollateral = 2,
    WithdrawCollateral = 3,
    Borrow = 4,
    Repay = 5,
    FillUserLiquidationAuction = 6,
    FillBadDebtAuction = 7,
    FillInterestAuction = 8,
    DeleteLiquidationAuction = 9,
}

pub fn default_reserve_metadata() -> ReserveConfig {
    ReserveConfig {
        decimals: 7,
        c_factor: 0_7500000,
        l_factor: 0_7500000,
        util: 0_7500000,
        max_util: 0_9500000,
        r_base: 0_0100000,
        r_one: 0_0500000,
        r_two: 0_5000000,
        r_three: 1_5000000,
        reactivity: 0_0000020, // 2e-6
        index: 0,
    }
}
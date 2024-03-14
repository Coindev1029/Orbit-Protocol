#![cfg(test)]

use sep_40_oracle::{Asset, PriceData};
use sep_41_token::{StellarAssetClient};
use soroban_sdk::{Env, Address, Vec, Val};

use super::*;

#[test]
fn test_oracle_bridge() {
    let env = Env::new();

}
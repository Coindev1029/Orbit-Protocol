
#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env};
use crate::{MockPegkeeperContract, MockPegkeeperClient};
extern crate std;

#[test]
pub fn test_set_get_treasury() {
    let e = Env::default();
    let contract_id = e.register_contract(None, MockPegkeeperContract);

    let client = MockPegkeeperClient::new(&e, &contract_id);
    let new_token_address = Address::generate(&e);
    let new_treasury_address = Address::generate(&e);
    client.add_treasury(&new_token_address, &new_treasury_address);
    assert_eq!(new_treasury_address, client.get_treasury(&new_token_address));
}

#[test]
pub fn test_set_get_maximum_duration() {
    let e = Env::default();
    let contract_id = e.register_contract(None, MockPegkeeperContract);

    let client = MockPegkeeperClient::new(&e, &contract_id);
    let new_maximum_duration = 1686150000;
    client.set_maximum_duration(&new_maximum_duration);
    assert_eq!(new_maximum_duration, client.get_maximum_duration());
}

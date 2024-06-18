
#![cfg(test)]

use soroban_sdk::testutils::Events;
use soroban_sdk::{testutils::Address as _, Address, Env, vec, Symbol, IntoVal, testutils::Logs};
use crate::{MockPegkeeperContract, MockPegkeeperClient};
use crate::dependencies::treasury::{Client as MockTreasuryClient, WASM as TREASURY_WASM};
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

#[test]
pub fn test_flash_loan_flow() {
    let e = Env::default();
    let mock_pegkeeper_id = e.register_contract(None, MockPegkeeperContract);
    let mock_treasury_id = e.register_contract_wasm(None, TREASURY_WASM);

    let mock_pegkeeper_client = MockPegkeeperClient::new(&e, &mock_pegkeeper_id);
    let mock_treasury_client = MockTreasuryClient::new(&e, &mock_treasury_id);
    let admin = Address::generate(&e);
    let token = Address::generate(&e);

    mock_treasury_client.set_pegkeeper(&mock_pegkeeper_id);
    mock_pegkeeper_client.initialize(&admin, &0_u64);
    // std::println!("  hhhhhhhh {:?} {:?}", token.clone().to_string(), mock_treasury_id.clone().to_string());
    mock_pegkeeper_client.add_treasury(&token, &mock_treasury_id);
    mock_pegkeeper_client.flash_loan(&token, &1000);

    let events = e.events().all();
    // println!("{:?}", e.events().all());
    assert_eq!(events.len(), 0);
    let logs = e.logs().all();
    std::println!("{}", logs.join("\n"));

    // let event = vec![&e, events.get_unchecked(events.len() - 1)];
    // assert_eq!(
    //     event,
    //     vec![
    //         &e,
    //         (
    //             mock_pegkeeper_id.clone(),
    //             (Symbol::new(&e, "flash_loan_receive"), token, 1000i128).into_val(&e),
    //             "Success".into_val(&e)
    //         )
    //     ]
    // );
    // assert_eq!(1, 2);
}
#![cfg(test)]
use soroban_sdk::{
    testutils::{Address as AddressTestTrait, Events, Logs},
    vec, Address, Error, IntoVal, Symbol, Val, Vec,
};
use test_suites::{
    create_fixture_with_data,
    test_fixture::{TokenIndex, SCALAR_7},
};
#[test]
fn test_mock_pegkeeper_flashloan() {

    let fixture = create_fixture_with_data();
    // let frodo: &Address = fixture.users.get(0).unwrap();
    // let pool_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

    let mock_usdt_token = &fixture.tokens[TokenIndex::OUSD];

    let token_balance_before = mock_usdt_token.balance(&fixture.mock_pegkeeper.address);
    fixture.mock_treasury.fl_loan(&1000i128);
    let token_balance_before = mock_usdt_token.balance(&fixture.mock_pegkeeper.address);
    assert_eq!(token_balance_before.clone(), token_balance_before.clone());
    std::println!("===================== balance{:?} =====================", mock_usdt_token.balance(&fixture.mock_pegkeeper.address));
    std::println!("=====================================FlashLoan Logs Start===========================================");
    std::println!("{:?}", fixture.env.logs().all().join("\n"));
    std::println!("=====================================FlashLoan Logs End===========================================");
}
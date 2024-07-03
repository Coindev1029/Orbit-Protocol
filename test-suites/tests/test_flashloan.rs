#![cfg(test)]
use soroban_sdk::{
    testutils::{Address as AddressTestTrait, Events, Logs},
    vec, Address, Error, IntoVal, Symbol, Val, Vec,
};
use test_suites::{
    create_fixture_with_data,
    test_fixture::{TokenIndex, SCALAR_7},
};
use test_suites::test_fixture::PoolFixture;

#[test]
fn test_mock_pegkeeper_flashloan() {

    let fixture = create_fixture_with_data(true);
    let ousd_client = &fixture.tokens[TokenIndex::OUSD];
    let caller = Address::generate(&fixture.env);
    let liquidation = Address::generate(&fixture.env);
    let treasury = &fixture.mock_treasury;
    let pegkeeper = fixture.mock_pegkeeper.address.clone();

    assert_eq!(
        0,
        ousd_client.balance(&pegkeeper)
    );

    // treasury.keep_peg(&caller, &ousd_client.address.clone(), &liquidation, &(1000 * SCALAR_7));

    // assert_eq!(
    //     1000 * SCALAR_7,
    //     ousd_client.balance(&pegkeeper)
    // );
}
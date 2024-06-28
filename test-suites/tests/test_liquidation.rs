#![cfg(test)]
use cast::i128;
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    testutils::{Address as AddressTestTrait, Events},
    vec, Address, Error, IntoVal, Symbol, Val, Vec,
};
use test_suites::{
    dependencies::pool::{Request, RequestType, Positions, PoolDataKey, ReserveConfig, ReserveData},
    assertions::assert_approx_eq_abs,
    create_fixture_with_data,
    test_fixture::{TokenIndex, SCALAR_7},
};

#[test]
fn test_liquidations() {
    let fixture = create_fixture_with_data(false);

    let pegkeeper = &fixture.mock_pegkeeper;

    let pool_fixture = &fixture.pools[0];

    let henk = Address::generate(&fixture.env);
    fixture.tokens[TokenIndex::XLM].mint(&henk, &(120_000 * SCALAR_7));

    let requests: Vec<Request> = vec![
        &fixture.env,
        Request {
            request_type: RequestType::SupplyCollateral as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: 100_000 * SCALAR_7,
        },
        Request {
            request_type: RequestType::Borrow as u32,
            address: fixture.tokens[TokenIndex::OUSD].address.clone(),
            amount: 8_800 * SCALAR_7,
        },
    ];
    pool_fixture.pool.submit(&henk, &henk, &henk, &requests);

    assert_eq!(
        20_000 * SCALAR_7,
        fixture.tokens[TokenIndex::XLM].balance(&henk)
    );
    assert_eq!(
        8_800 * SCALAR_7,
        fixture.tokens[TokenIndex::OUSD].balance(&henk)
    );

    fixture.jump(60 * 60 * 24 * 7 * 4); // 4 weeks
    fixture.oracle.set_price_stable(&vec![
        &fixture.env,
        1_0000000,    // usdc
        0_0800000,    // xlm
    ]);   
    
    let liq_pct = 100;
    let auction_data = pool_fixture
        .pool
        .new_liquidation_auction(&henk, &liq_pct);

    let ousd_bid_amount = auction_data.bid.get_unchecked(fixture.tokens[TokenIndex::OUSD].address.clone());
    let xlm_lot_amount = auction_data.lot.get_unchecked(fixture.tokens[TokenIndex::XLM].address.clone());
    //assert_approx_eq_abs(ousd_bid_amount, 19599_872330, 100000);

    std::println!("ousd_bid_amount: {}", ousd_bid_amount);
    std::println!("ousd_lot_amount: {}", xlm_lot_amount);
    //allow 250 blocks to pass
    fixture.jump_with_sequence(251 * 5);

    let piet = Address::generate(&fixture.env);
    fixture.tokens[TokenIndex::OUSD].mint(&pegkeeper.address.clone(), &(10_000 * SCALAR_7));
    
    pegkeeper.liquidate(&henk, &fixture.tokens[TokenIndex::OUSD].address.clone(), &ousd_bid_amount, &fixture.tokens[TokenIndex::XLM].address.clone(), &xlm_lot_amount, &pool_fixture.pool.address.clone(), &(100 as i128));

    assert_eq!(
        xlm_lot_amount,
        fixture.tokens[TokenIndex::XLM].balance(&pegkeeper.address.clone())
    );
}
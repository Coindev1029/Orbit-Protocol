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
    ];
    pool_fixture.pool.submit(&henk, &henk, &henk, &requests);

    assert_eq!(
        20_000 * SCALAR_7,
        fixture.tokens[TokenIndex::XLM].balance(&henk)
    );

    let requests: Vec<Request> = vec![
        &fixture.env,
        Request {
            request_type: RequestType::Borrow as u32,
            address: fixture.tokens[TokenIndex::OUSD].address.clone(),
            amount: 8_800 * SCALAR_7,
        },
    ];
    pool_fixture.pool.submit(&henk, &henk, &henk, &requests);

    assert_eq!(
        8_800 * SCALAR_7,
        fixture.tokens[TokenIndex::OUSD].balance(&henk)
    );

    fixture.jump(60 * 60 * 24 * 7 * 4); // 4 weeks
    fixture.oracle.set_price_stable(&vec![
        &fixture.env,
        1_0000000,    // usdc
        0_0880000,    // xlm
    ]);

    //fully liquidate user
    let blank_requests: Vec<Request> = vec![&fixture.env];
    pool_fixture
        .pool
        .submit(&henk, &henk, &henk, &blank_requests);
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

    let fill_requests = vec![
        &fixture.env,
        Request {
            request_type: RequestType::FillUserLiquidationAuction as u32,
            address: henk.clone(),
            amount: 100,
        },
        Request {
            request_type: RequestType::Repay as u32,
            address: fixture.tokens[TokenIndex::OUSD].address.clone(),
            amount: ousd_bid_amount,
        },
        Request {
            request_type: RequestType::WithdrawCollateral as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: xlm_lot_amount,
        },
    ];

    let piet = Address::generate(&fixture.env);
    fixture.tokens[TokenIndex::OUSD].mint(&piet, &(8_800 * SCALAR_7));

    std::println!("balance xlm before: {}", fixture.tokens[TokenIndex::XLM].balance(&piet) / SCALAR_7);
    std::println!("balance ousd before: {}", fixture.tokens[TokenIndex::OUSD].balance(&piet) / SCALAR_7);
    pool_fixture
        .pool
        .submit(&piet, &piet, &piet, &fill_requests);

    assert_eq!(
        xlm_lot_amount,
        fixture.tokens[TokenIndex::XLM].balance(&piet)
    );

    std::println!("balance xlm after: {}", fixture.tokens[TokenIndex::XLM].balance(&piet) / SCALAR_7);
    std::println!("balance ousd after: {}", fixture.tokens[TokenIndex::OUSD].balance(&piet) / SCALAR_7);
}

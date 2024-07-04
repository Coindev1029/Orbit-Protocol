#![cfg(test)]
use cast::i128;
use soroban_fixed_point_math::FixedPoint;
use soroban_sdk::{
    log, testutils::{Address as AddressTestTrait, Events, Logs}, vec, Address, Error, IntoVal, Symbol, Val, Vec
};
use test_suites::{
    dependencies::pool::{Request, RequestType, Positions, PoolDataKey, ReserveConfig, ReserveData},
    assertions::assert_approx_eq_abs,
    create_fixture_with_data,
    test_fixture::{TokenIndex, SCALAR_7},
};

#[test]
fn test_liquidations() {
    let mut fixture = create_fixture_with_data(true);

    let initial_xlm_amount = 10_000_000_000_00 * SCALAR_7; // Assuming 1 XLM
    let initial_ousd_amount = (initial_xlm_amount as f64 * 0.088) as i128;
    fixture.create_pair(TokenIndex::OUSD, TokenIndex::XLM, initial_ousd_amount, initial_xlm_amount);

    let pool_fixture = &fixture.pools[0];
    let henk = Address::generate(&fixture.env);
    let treasury = &fixture.mock_treasury;

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
        0_0880000,    // xlm
    ]);

    // Create the token pair with initial supply.

    let pegkeeper = &fixture.mock_pegkeeper;

    let liq_pct = 100;
    let auction_data = pool_fixture
        .pool
        .new_liquidation_auction(&henk, &liq_pct);

    let ousd_bid_amount = auction_data.bid.get_unchecked(fixture.tokens[TokenIndex::OUSD].address.clone());
    let xlm_lot_amount = auction_data.lot.get_unchecked(fixture.tokens[TokenIndex::XLM].address.clone());

    //allow 250 blocks to pass
    fixture.jump_with_sequence(251 * 5);

    let pair = &fixture.pairs[0];

    treasury.keep_peg(&pair.address.clone(), &henk, &fixture.tokens[TokenIndex::OUSD].address.clone(), &ousd_bid_amount, &fixture.tokens[TokenIndex::XLM].address.clone(), &xlm_lot_amount,  &(100 as i128));

    std::println!("OUSD Balance: {}", fixture.tokens[TokenIndex::OUSD].balance(&pegkeeper.address.clone()) / SCALAR_7);
    std::println!("XLM Balance: {}", fixture.tokens[TokenIndex::XLM].balance(&pegkeeper.address.clone()));

    let logs = fixture.env.logs().all();
    std::println!("{}", logs.join("\n"));
    // Check if the liquidation has completed succesfully.
}

#[test]
fn test_liquidations_real() {
    let mut fixture = create_fixture_with_data(false);

    let initial_xlm_amount = 10_000_000_000_00 * SCALAR_7; // Assuming 1 XLM
    let initial_ousd_amount = (initial_xlm_amount as f64 * 0.088) as i128;
    fixture.create_pair(TokenIndex::OUSD, TokenIndex::XLM, initial_ousd_amount, initial_xlm_amount);

    let pool_fixture = &fixture.pools[0];
    let henk = Address::generate(&fixture.env);
    let treasury = &fixture.treasury;

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
        0_0880000,    // xlm
    ]);

    // Create the token pair with initial supply.

    let pegkeeper = &fixture.pegkeeper;

    let liq_pct = 100;
    let auction_data = pool_fixture
        .pool
        .new_liquidation_auction(&henk, &liq_pct);

    let ousd_bid_amount = auction_data.bid.get_unchecked(fixture.tokens[TokenIndex::OUSD].address.clone());
    let xlm_lot_amount = auction_data.lot.get_unchecked(fixture.tokens[TokenIndex::XLM].address.clone());

    //allow 250 blocks to pass
    fixture.jump_with_sequence(251 * 5);

    let pair = &fixture.pairs[0];

    treasury.keep_peg(&pair.address.clone(), &henk, &fixture.tokens[TokenIndex::OUSD].address.clone(), &ousd_bid_amount, &fixture.tokens[TokenIndex::XLM].address.clone(), &xlm_lot_amount,  &(100 as i128));

    std::println!("OUSD Balance: {}", fixture.tokens[TokenIndex::OUSD].balance(&pegkeeper.address.clone()) / SCALAR_7);
    std::println!("XLM Balance: {}", fixture.tokens[TokenIndex::XLM].balance(&pegkeeper.address.clone()));

    let logs = fixture.env.logs().all();
    std::println!("{}", logs.join("\n"));
    // Check if the liquidation has completed succesfully.
}


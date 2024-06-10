use soroban_sdk::{testutils::Address as _, vec as svec, Address, Symbol};

use crate::{
    dependencies::pool::{default_reserve_metadata, ReserveEmissionMetadata},
    test_fixture::{TestFixture, TokenIndex, SCALAR_7},
};
use crate::dependencies::pool::{Request, RequestType, ReserveConfig};

/// Create a test fixture with a pool and a whale depositing and borrowing all assets
pub fn create_fixture_with_data<'a>() -> TestFixture<'a> {
    let mut fixture = TestFixture::create();

    // mint whale tokens
    let frodo = Address::generate(&fixture.env);
    fixture.users.push(frodo.clone());
    fixture.tokens[TokenIndex::XLM].mint(&frodo, &(10_000_000_000 * SCALAR_7)); // 10B XLM

    // mint LP tokens with whale
    fixture.tokens[TokenIndex::BLND].mint(&frodo, &(500_0010_000_0000_0000 * SCALAR_7));
    // fixture.tokens[TokenIndex::BLND].approve(&frodo, &fixture.lp.address, &i128::MAX, &99999);
    fixture.tokens[TokenIndex::USDC].mint(&frodo, &(12_5010_000_0000_0000 * SCALAR_7));
    // fixture.tokens[TokenIndex::USDC].approve(&frodo, &fixture.lp.address, &i128::MAX, &99999);
    fixture.lp.join_pool(
        &(500_000_0000 * SCALAR_7),
        &svec![
            &fixture.env,
            500_0010_000_0000_0000 * SCALAR_7,
            12_5010_000_0000_0000 * SCALAR_7,
        ],
        &frodo,
    );

    fixture.create_pool(Symbol::new(&fixture.env, "Teapot"), 0_1000000, 6);

    let ousd_config = ReserveConfig {
        decimals: 7,
        c_factor: 0,
        l_factor: 1_000_0000,
        util: 0_800_0000,
        max_util: 1_000_0000,
        r_one: 0_040_0000,
        r_two: 0_200_0000,
        r_three: 0_790_0000,
        reactivity: 0_0000200, // 2e-5
        index: 0,
    };
    let xlm_config = ReserveConfig {
        decimals: 7,
        c_factor: 0_890_0000,
        l_factor: 0,
        util: 0,
        max_util: 1_000_0000,
        r_one: 0_040_0000,
        r_two: 0_200_0000,
        r_three: 0_790_0000,
        reactivity: 0_0000200, // 2e-5
        index: 1,
    };
    fixture.create_pool_reserve(0, TokenIndex::OUSD, &ousd_config);
    fixture.create_pool_reserve(0, TokenIndex::XLM, &xlm_config);

    // enable emissions for pool
    let pool_fixture = &fixture.pools[0];
    let reserve_emissions: soroban_sdk::Vec<ReserveEmissionMetadata> = svec![
        &fixture.env,
        ReserveEmissionMetadata {
            res_index: 0, // Orbit
            res_type: 0,  // d_token
            share: 0_600_0000
        },
        ReserveEmissionMetadata {
            res_index: 1, // XLM
            res_type: 1,  // b_token
            share: 0_400_0000
        },
    ];
    pool_fixture.pool.set_emissions_config(&reserve_emissions);

    // deposit into backstop, add to reward zone
    fixture
        .backstop
        .deposit(&frodo, &pool_fixture.pool.address, &(50_000 * SCALAR_7));
    fixture.backstop.update_tkn_val();
    fixture
        .backstop
        .add_reward(&pool_fixture.pool.address, &Address::generate(&fixture.env));
    pool_fixture.pool.set_status(&3);
    pool_fixture.pool.update_status();

    // enable emissions
    fixture.emitter.distribute();
    fixture.backstop.gulp_emissions();
    pool_fixture.pool.gulp_emissions();

    fixture.jump(60);

    // fixture.tokens[TokenIndex::XLM].approve(&frodo, &pool_fixture.pool.address, &i128::MAX, &50000);

    pool_fixture.treasury.increase_supply(&(100_000_000 * SCALAR_7)); // Treasury supplies 100M stable to pool

    //fixture.create_pair(TokenIndex::OUSD, TokenIndex::USDC);
    //let pair = &fixture.pairs[0].pair;

    // let deposit_amount = 6_000_0000 * SCALAR_7;
    // fixture.tokens[TokenIndex::OUSD].mint(&pair.address, &(deposit_amount));
    // fixture.tokens[TokenIndex::USDC].mint(&pair.address, &(deposit_amount));
    // pair.deposit(&frodo);

    let henk = Address::generate(&fixture.env);
    fixture.users.push(henk.clone());
    fixture.tokens[TokenIndex::XLM].mint(&henk, &(100_000 * SCALAR_7)); // 100k XLM

    let requests = svec![
        &fixture.env,
        Request {
            request_type: RequestType::SupplyCollateral as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: 50_000 * SCALAR_7,
        },
        Request {
            request_type: RequestType::Borrow as u32,
            address: fixture.tokens[TokenIndex::OUSD].address.clone(),
            amount: 1_000 * SCALAR_7,
        },
    ];
    pool_fixture.pool.submit(&henk, &henk, &henk, &requests);

    fixture.jump(60 * 60); // 1 hr

    fixture.env.budget().reset_unlimited();
    fixture
}

#[cfg(test)]
mod tests {
    use crate::test_fixture::{PoolFixture};

    use super::*;

    //TODO: Emmissions check?

    #[test]
    fn test_create_fixture_with_data_wasm() {
        let fixture: TestFixture<'_> = create_fixture_with_data();
        let frodo = fixture.users.get(0).unwrap();
        let henk = fixture.users.get(1).unwrap();
        let treasury_fixture: &PoolFixture = fixture.pools.get(0).unwrap();
        //let pair = &fixture.pairs[0].pair;

        // validate backstop deposit
        assert_eq!(
            50_000 * SCALAR_7,
            fixture.lp.balance(&fixture.backstop.address)
        );

        // validate collateral deposit
        assert_eq!(
            50_000 * SCALAR_7,
            fixture.tokens[TokenIndex::XLM].balance(&henk)
        );

        // validate borrow
        assert_eq!(
            1_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&henk)
        );
    }
}

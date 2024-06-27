use soroban_sdk::{log, testutils::{Address as _, Logs, MockAuth, MockAuthInvoke}, vec as svec, Address, String, Symbol, Env, Vec as SVec};

use crate::{
    dependencies::pool::{default_reserve_metadata, Request, RequestType, ReserveEmissionMetadata},
    test_fixture::{TestFixture, TokenIndex, SCALAR_7},
    dependencies::mock_treasury::MockAsset,
    dependencies::treasury::Asset,
};
use crate::dependencies::pool::ReserveConfig;

/// Create a test fixture with a pool and a whale depositing and borrowing all assets
pub fn create_fixture_with_data<'a>(mock: bool) -> TestFixture<'a> {
    std::println!("===================================== Fixture Create With Data ===========================================");

    let mut fixture = TestFixture::create(mock);

    // mint whale tokens
    let frodo = fixture.users[0].clone();
    fixture.users.push(frodo.clone());

    fixture.tokens[TokenIndex::XLM].mint(&frodo, &(10_000_000_000 * SCALAR_7)); // 10B XLM

    // mint LP tokens with whale
    fixture.tokens[TokenIndex::BLND].mint(&frodo, &(500_0010_000_0000_0000 * SCALAR_7));
    // fixture.tokens[TokenIndex::BLND].approve(&frodo, &fixture.lp.address, &i128::MAX, &99999);
    fixture.tokens[TokenIndex::USDC].mint(&frodo, &(12_5010_000_0000_0000 * SCALAR_7));
    // fixture.tokens[TokenIndex::USDC].approve(&frodo, &fixture.lp.address, &i128::MAX, &
    fixture.lp.join_pool(
        &(500_000_0000 * SCALAR_7),
        &svec![
            &fixture.env,
            500_0010_000_0000_0000 * SCALAR_7,
            12_5010_000_0000_0000 * SCALAR_7,
        ],
        &frodo,
    );

    fixture.create_pool(String::from_str(&fixture.env, "Teapot"), 0_9999999, 6);

    let ousd_config = ReserveConfig {
        decimals: 7,
        c_factor: 0,
        l_factor: 1_000_0000,
        util: 0_800_0000,
        max_util: 1_000_0000,
        r_base: 0_040_0000,
        r_one: 0,
        r_two: 0,
        r_three: 0,
        reactivity: 0, // 2e-5
        index: 0,
    };
    let xlm_config = ReserveConfig {
        decimals: 7,
        c_factor: 0_890_0000,
        l_factor: 0,
        util: 0,
        max_util: 1_000_0000,
        r_base: 0_040_0000,
        r_one: 0,
        r_two: 0,
        r_three: 0,
        reactivity: 0,
        index: 1,
    };

    fixture.create_pool_reserve(0, TokenIndex::XLM, &xlm_config);
    fixture.create_pool_reserve(0, TokenIndex::OUSD, &ousd_config);

    // enable emissions for pool
    let pool_fixture = &fixture.pools[0];

    let reserve_emissions: soroban_sdk::Vec<ReserveEmissionMetadata> = soroban_sdk::vec![
        &fixture.env,
        ReserveEmissionMetadata {
            res_index: 0, // OUSD
            res_type: 0,  // d_token
            share: 0_600_0000,
        },
        ReserveEmissionMetadata {
            res_index: 1, // XLM
            res_type: 1,  // b_token
            share: 0_400_0000,
        },
    ];
    pool_fixture.pool.set_emissions_config(&reserve_emissions);

    // initiate the Treasury
    let token: Address = fixture.tokens[TokenIndex::OUSD].address.clone();
    if (mock) {
        let asset: MockAsset = MockAsset::Stellar(fixture.tokens[TokenIndex::USDC].address.clone());
        fixture.mock_treasury.deploy_stablecoin(&token, &asset, &pool_fixture.pool.address);
        fixture.tokens[TokenIndex::OUSD].set_admin(&fixture.mock_treasury.address);
    } else {
        let asset: Asset = Asset::Stellar(fixture.tokens[TokenIndex::USDC].address.clone());
        fixture.treasury.deploy_stablecoin(&token, &asset, &pool_fixture.pool.address);
        fixture.tokens[TokenIndex::OUSD].set_admin(&fixture.treasury.address);
    }


    // deposit into backstop, add to reward zone
    fixture
        .backstop
        .deposit(&frodo, &pool_fixture.pool.address, &(50_000 * SCALAR_7));

    fixture.backstop.update_tkn_val();

    fixture
        .backstop
        .add_reward(&pool_fixture.pool.address, &Address::generate(&fixture.env));
    pool_fixture.pool.set_status(&0);
    pool_fixture.pool.update_status();

    // enable emissions
    fixture.emitter.distribute();
    fixture.backstop.gulp_emissions();
    pool_fixture.pool.gulp_emissions();

    fixture.jump(60);

    if (mock) {
        fixture.mock_treasury.increase_supply(&token, &(1_000_000 * SCALAR_7));
    } else {
        fixture.treasury.increase_supply(&token, &(1_000_000 * SCALAR_7));
    }

    std::println!("===================================== Fixture Create With Data Successfully ===========================================");

    fixture.jump(60 * 60); // 1 hr

    fixture.env.budget().reset_unlimited();
    fixture
}

#[cfg(test)]
mod tests {
    use crate::test_fixture::PoolFixture;

    use super::*;

    #[test]
    fn test_create_fixture_with_data_wasm_mock() {
        let fixture: TestFixture<'_> = create_fixture_with_data(true);
        let frodo: &Address = fixture.users.get(0).unwrap();
        let pool_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

        // validate backstop deposit and drop
        assert_eq!(
            50_000 * SCALAR_7,
            fixture.lp.balance(&fixture.backstop.address)
        );
        assert_eq!(
            10_000_000 * SCALAR_7,
            fixture.tokens[TokenIndex::BLND].balance(&fixture.admin)
        );

        //Check treasury supply
        assert_eq!(
            1_000_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&pool_fixture.pool.address)
        );

        let henk = Address::generate(&fixture.env);
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

        assert_eq!(
            50_000 * SCALAR_7,
            fixture.tokens[TokenIndex::XLM].balance(&pool_fixture.pool.address)
        );
        assert_eq!(
            1_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&henk)
        );
    }

    #[test]
    fn test_create_fixture_with_data_wasm() {
        let fixture: TestFixture<'_> = create_fixture_with_data(false);
        let frodo: &Address = fixture.users.get(0).unwrap();
        let pool_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

        // validate backstop deposit and drop
        assert_eq!(
            50_000 * SCALAR_7,
            fixture.lp.balance(&fixture.backstop.address)
        );
        assert_eq!(
            10_000_000 * SCALAR_7,
            fixture.tokens[TokenIndex::BLND].balance(&fixture.admin)
        );

        //Check treasury supply
        assert_eq!(
            1_000_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&pool_fixture.pool.address)
        );

        let henk = Address::generate(&fixture.env);
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

        assert_eq!(
            50_000 * SCALAR_7,
            fixture.tokens[TokenIndex::XLM].balance(&pool_fixture.pool.address)
        );
        assert_eq!(
            1_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&henk)
        );
    }
}

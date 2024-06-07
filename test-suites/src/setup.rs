use soroban_sdk::{testutils::Address as _, vec as svec, Address, Symbol, Vec as SVec, String};

use crate::{
    dependencies::pool::{default_reserve_metadata, RequestType, ReserveEmissionMetadata, Request},
    test_fixture::{TestFixture, TokenIndex, SCALAR_7},
};

/// Create a test fixture with a pool and a whale depositing and borrowing all assets
pub fn create_fixture_with_data<'a>() -> TestFixture<'a> {
    let mut fixture = TestFixture::create();

    // mint whale tokens
    let frodo = Address::generate(&fixture.env);
    fixture.users.push(frodo.clone());
    fixture.tokens[TokenIndex::XLM].mint(&frodo, &(1_000_000 * SCALAR_7));

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

    let mut ousd_config = default_reserve_metadata();
    ousd_config.c_factor = 0_900_0000;
    ousd_config.l_factor = 0_950_0000;
    ousd_config.util = 0_850_0000;
    fixture.create_pool_reserve(0, TokenIndex::OUSD, &ousd_config);

    let mut xlm_config = default_reserve_metadata();
    xlm_config.c_factor = 0_750_0000;
    xlm_config.l_factor = 0_750_0000;
    xlm_config.util = 0_500_0000;
    fixture.create_pool_reserve(0, TokenIndex::XLM, &xlm_config);

    // enable emissions for pool
    let treasury_fixture = &fixture.pools[0];
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
    treasury_fixture.pool.set_emissions_config(&reserve_emissions);

    // deposit into backstop, add to reward zone
    fixture
        .backstop
        .deposit(&frodo, &treasury_fixture.pool.address, &(50_000 * SCALAR_7));
    fixture.backstop.update_tkn_val();
    fixture
        .backstop
        .add_reward(&treasury_fixture.pool.address, &Address::generate(&fixture.env));
    treasury_fixture.pool.set_status(&3);
    treasury_fixture.pool.update_status();

    // enable emissions
    fixture.emitter.distribute();
    fixture.backstop.gulp_emissions();
    treasury_fixture.pool.gulp_emissions();

    fixture.jump(60);

    // fixture.tokens[TokenIndex::XLM].approve(&frodo, &pool_fixture.pool.address, &i128::MAX, &50000);

    treasury_fixture.treasury.increase_supply(&(100_000 * SCALAR_7)); // Treasury supplies 100k stable to pool


    //supply and borrow STABLE for 80% utilization (close to target)
    // let requests: SVec<Request> = svec![
    //     &fixture.env,
    //     Request {
    //         request_type: RequestType::SupplyCollateral as u32,
    //         address: fixture.tokens[TokenIndex::XLM].address.clone(),
    //         amount: 100_000 * SCALAR_7,
    //     },
    //     Request {
    //         request_type: RequestType::Borrow as u32,
    //         address: fixture.tokens[TokenIndex::XLM].address.clone(),
    //         amount: 60_000 * SCALAR_7,
    //     },
    // ];
    // pool_fixture.pool.submit(&frodo, &frodo, &frodo, &requests);


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
        let treasury_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

        // validate backstop deposit
        assert_eq!(
            50_000 * SCALAR_7,
            fixture.lp.balance(&fixture.backstop.address)
        );

        // validate pool actions
        assert_eq!(
            100_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&treasury_fixture.pool.address)
        );
        treasury_fixture.treasury.increase_supply(&(100_000 * SCALAR_7)); // Treasury supplies 100k stable to pool
        assert_eq!(
            200_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&treasury_fixture.pool.address)
        );
        treasury_fixture.treasury.decrease_supply(&(100_000 * SCALAR_7)); // Treasury supplies 100k stable to pool
        assert_eq!(
            100_000 * SCALAR_7,
            fixture.tokens[TokenIndex::OUSD].balance(&treasury_fixture.pool.address)
        );
        assert_eq!(0, fixture.tokens[TokenIndex::OUSD].balance(&treasury_fixture.treasury.address));
    }
}

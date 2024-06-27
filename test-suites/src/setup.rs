use soroban_sdk::{log, testutils::{Address as _, Logs, MockAuth, MockAuthInvoke}, vec as svec, Address, String, Symbol, Env, Vec as SVec};

use crate::{
    dependencies::pool::{default_reserve_metadata, Request, RequestType, ReserveEmissionMetadata},
    test_fixture::{TestFixture, TokenIndex, SCALAR_7},
};

/// Create a test fixture with a pool and a whale depositing and borrowing all assets
pub fn create_fixture_with_data<'a>() -> TestFixture<'a> {

    std::println!("===================================== Fixture Create With Data ===========================================");

    let mut fixture = TestFixture::create();

    std::println!("===================================== After Create Function ===========================================");

    // mint whale tokens
    let frodo = fixture.users[0].clone();
    fixture.users.push(frodo.clone());
    fixture.tokens[TokenIndex::XLM].mint(&frodo, &(1_000_000 * SCALAR_7)); // 10B XLM

    std::println!("===================================== After XLM Mint ===========================================");

    // mint LP tokens with whale
    fixture.tokens[TokenIndex::BLND].mint(&frodo, &(70_000_000 * SCALAR_7));
    // fixture.tokens[TokenIndex::BLND].approve(&frodo, &fixture.lp.address, &i128::MAX, &99999);
    fixture.tokens[TokenIndex::MockOusd].mint(&frodo, &(2_600_000 * SCALAR_7));
    // fixture.tokens[TokenIndex::USDC].approve(&frodo, &fixture.lp.address, &i128::MAX, &99999);

    std::println!("===================================== After Blend, MockOusd Mint ===========================================");

    fixture.lp.join_pool(
        &(10_000_000 * SCALAR_7),
        &svec![
            &fixture.env,
            110_000_000 * SCALAR_7,
            2_600_000 * SCALAR_7,
        ],
        &frodo,
    );

    std::println!("===================================== After LP Join Pool ===========================================");

    fixture.create_pool(String::from_str(&fixture.env, "Teapot"), 0_1000000, 6);

    std::println!("===================================== After Create Pool ===========================================");

    let mut mock_ousd_config = default_reserve_metadata();
    mock_ousd_config.decimals = 6;
    mock_ousd_config.c_factor = 0_900_0000;
    mock_ousd_config.l_factor = 0_950_0000;
    mock_ousd_config.util = 0_850_0000;
    fixture.create_pool_reserve(0, TokenIndex::MockOusd, &mock_ousd_config);

    let mut xlm_config = default_reserve_metadata();
    xlm_config.c_factor = 0_750_0000;
    xlm_config.l_factor = 0_750_0000;
    xlm_config.util = 0_500_0000;
    fixture.create_pool_reserve(0, TokenIndex::XLM, &xlm_config);

    // enable emissions for pool
    let pool_fixture = &fixture.pools[0];
    let reserve_emissions: soroban_sdk::Vec<ReserveEmissionMetadata> = svec![
        &fixture.env,
        ReserveEmissionMetadata {
            res_index: 0, // ousd
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
    
    std::println!("=====================================Request Test===========================================");
    // supply and borrow MockOusd for 80% utilization (close to target)
    let requests: SVec<Request> = svec![
        &fixture.env,
        Request {
            request_type: RequestType::SupplyCollateral as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: 100_000 * SCALAR_7,
        },
        Request {
            request_type: RequestType::Borrow as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: 65_000 * SCALAR_7,
        },
    ];

    std::println!("=====================================Before Submit===========================================");
    
    std::println!("=====================================Address {:?}===========================================", &pool_fixture.pool.address);
    
    pool_fixture.pool.submit(&frodo, &frodo, &frodo, &requests);

    std::println!("=====================================Request Test===========================================");
    // supply and borrow MockOusd for 80% utilization (close to target)
    let requests: SVec<Request> = svec![
        &fixture.env,
        Request {
            request_type: RequestType::Borrow as u32,
            address: fixture.tokens[TokenIndex::XLM].address.clone(),
            amount: 65_000 * SCALAR_7,
        },
    ];

    std::println!("=====================================Before Submit===========================================");
    
    std::println!("=====================================Address {:?}===========================================", &pool_fixture.pool.address);
    
    pool_fixture.pool.submit(&frodo, &frodo, &frodo, &requests);

    std::println!("=====================================After Submit===========================================");

    // supply and borrow XLM for 65% utilization (above target)
    let requests: SVec<Request> = svec![
        &fixture.env,
        Request {
            request_type: RequestType::SupplyCollateral as u32,
            address: fixture.tokens[TokenIndex::MockOusd].address.clone(),
            amount: 10 * 10i128.pow(9),
        },
        // Request {
        //     request_type: RequestType::Borrow as u32,
        //     address: fixture.tokens[TokenIndex::MockOusd].address.clone(),
        //     amount: 5 * 10i128.pow(9),
        // },
    ];
    pool_fixture.pool.submit(&frodo, &frodo, &frodo, &requests);

    std::println!("=====================================Init Test===========================================");

    fixture.jump(60 * 60); // 1 hr

    fixture.env.budget().reset_unlimited();
    fixture
}

#[cfg(test)]
mod tests {
    use crate::test_fixture::PoolFixture;

    use super::*;

    // #[test]
    // fn test_create_fixture_with_data_wasm() {
    //     let fixture: TestFixture<'_> = create_fixture_with_data();
    //     let frodo: &Address = fixture.users.get(0).unwrap();
    //     let pool_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

    //     // validate backstop deposit and drop
    //     assert_eq!(
    //         50_000 * SCALAR_7,
    //         fixture.lp.balance(&fixture.backstop.address)
    //     );
    //     assert_eq!(
    //         10_000_000 * SCALAR_7,
    //         fixture.tokens[TokenIndex::BLND].balance(&fixture.admin)
    //     );

    //     // validate pool actions
    //     assert_eq!(
    //         2_000 * 10i128.pow(6),
    //         fixture.tokens[TokenIndex::MockOusd].balance(&pool_fixture.pool.address)
    //     );
    //     assert_eq!(
    //         35_000 * SCALAR_7,
    //         fixture.tokens[TokenIndex::XLM].balance(&pool_fixture.pool.address)
    //     );

    //     assert_eq!(
    //         965_000 * SCALAR_7,
    //         fixture.tokens[TokenIndex::XLM].balance(&frodo)
    //     );
    //     // validate emissions are turned on
    //     let (emis_config, emis_data) = fixture.read_reserve_emissions(0, TokenIndex::MockOusd, 0);
    //     assert_eq!(
    //         emis_data.last_time,
    //         fixture.env.ledger().timestamp() - 60 * 61
    //     );
    //     assert_eq!(emis_data.index, 0);
    //     assert_eq!(0_180_0000, emis_config.eps);
    //     assert_eq!(
    //         fixture.env.ledger().timestamp() + 7 * 24 * 60 * 60 - 60 * 61,
    //         emis_config.expiration
    //     )
    // }

    #[test]
    fn test_mock_pegkeeper_flashloan() {
        // use crate::test_fixture::PoolFixture;

        use super::*;

        let fixture: TestFixture<'_> = create_fixture_with_data();
        // let frodo: &Address = fixture.users.get(0).unwrap();
        // let pool_fixture: &PoolFixture = fixture.pools.get(0).unwrap();

        let mock_usdt_token = &fixture.tokens[TokenIndex::MockOusd];

        let token_balance_before = mock_usdt_token.balance(&fixture.mock_receiver.address);
        fixture.mock_pegkeeper.flash_loan(&mock_usdt_token.address, &1000i128);
        let token_balance_before = mock_usdt_token.balance(&fixture.mock_receiver.address);
        assert_eq!(token_balance_before.clone(), token_balance_before.clone());
        std::println!("===================== balance{:?} =====================", mock_usdt_token.balance(&fixture.mock_receiver.address));
        std::println!("=====================================FlashLoan Logs Start===========================================");
        std::println!("{:?}", fixture.env.logs().all().join("\n"));
        std::println!("=====================================FlashLoan Logs End===========================================");
    }
}

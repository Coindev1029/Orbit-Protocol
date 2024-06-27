use std::collections::HashMap;
use std::ops::Index;

use crate::dependencies::backstop::create_backstop;
use crate::dependencies::emitter::create_emitter;
use crate::dependencies::liquidity_pool::{create_lp_pool, LPClient};
use crate::dependencies::oracle::create_mock_oracle;
use crate::dependencies::pool::POOL_WASM;
use crate::dependencies::pool_factory::create_pool_factory;
use crate::dependencies::token::{create_stellar_token, create_token};
use crate::dependencies::backstop::BackstopClient;
use crate::dependencies::emitter::EmitterClient;
use crate::dependencies::pool::{
    PoolClient, PoolConfig, PoolDataKey, ReserveConfig, ReserveData, ReserveEmissionsConfig,
    ReserveEmissionsData,
};
use crate::dependencies::pool_factory::{PoolFactoryClient, PoolInitMeta};
use sep_40_oracle::testutils::{Asset, MockPriceOracleClient};
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, BytesN as _, Ledger, LedgerInfo};
use soroban_sdk::{vec as svec, Address, BytesN, Env, String, Map, Symbol};

use crate::dependencies::pair::{PAIR_WASM, PairClient};
use crate::dependencies::treasury::{TreasuryClient, TREASURY_WASM};
use crate::dependencies::bridge_oracle::{BridgeOracleClient, create_bridge_oracle};
use crate::dependencies::pair_factory::{create_pair_factory, PairFactoryClient};
use crate::dependencies::router::{create_router, RouterClient};
use crate::dependencies::mock_treasury::{create_mock_treasury, MockTreasuryClient};
use crate::dependencies::mock_pegkeeper::{create_mock_pegkeeper, MockPegkeeperClient};

pub const SCALAR_7: i128 = 1_000_0000;
pub const SCALAR_9: i128 = 1_000_000_000;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum TokenIndex {
    BLND = 0,
    XLM = 1,
    MockOusd = 2
}

pub struct PoolFixture<'a> {
    pub treasury: TreasuryClient<'a>,
    pub pool: PoolClient<'a>,
    pub reserves: HashMap<TokenIndex, u32>,
}

pub struct PairFixture<'a> {
    pub pair: PairClient<'a>,
}

impl<'a> Index<TokenIndex> for Vec<MockTokenClient<'a>> {
    type Output = MockTokenClient<'a>;

    fn index(&self, index: TokenIndex) -> &Self::Output {
        &self[index as usize]
    }
}

pub struct TestFixture<'a> {
    pub env: Env,
    pub admin: Address,
    pub users: Vec<Address>,
    pub emitter: EmitterClient<'a>,
    pub backstop: BackstopClient<'a>,
    pub pool_factory: PoolFactoryClient<'a>,
    pub oracle: MockPriceOracleClient<'a>,
    pub lp: LPClient<'a>,
    pub pools: Vec<PoolFixture<'a>>,
    pub tokens: Vec<MockTokenClient<'a>>,
    pub pair_factory: PairFactoryClient<'a>,
    pub pairs: Vec<PairFixture<'a>>,   
    pub router: RouterClient<'a>,
    pub bridge_oracle: BridgeOracleClient<'a>,          
    pub mock_treasury: MockTreasuryClient<'a>,
    pub mock_pegkeeper: MockPegkeeperClient<'a>,
}

impl TestFixture<'_> {
    /// Create a new TestFixture for the Orbit Protocol
    ///
    /// Deploys BLND (0), USDC (1), wETH (2), XLM (3), and STABLE (4) test tokens, alongside all required
    /// Blend Protocol dependencies, including a BLND-USDC LP.
    
    pub fn create<'a>() -> TestFixture<'a> {

        std::println!("===================================== Fixture Create in Text Fixture ===========================================");

        let e = Env::default();
        e.mock_all_auths();
        e.budget().reset_unlimited();

        let admin = Address::generate(&e);
        let frodo = Address::generate(&e);

        e.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 (backstop epoch)
            protocol_version: 20,
            sequence_number: 150,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 500000,
            min_persistent_entry_ttl: 500000,
            max_entry_ttl: 9999999,
        });

        let (blnd_id, blnd_client) = create_stellar_token(&e, &admin);
        let (usdc_id, usdc_client) = create_stellar_token(&e, &admin);
        let (mock_ousd_token_id, mock_ousd_token_client) = create_stellar_token(&e, &admin);
        let (xlm_id, xlm_client) = create_stellar_token(&e, &admin);

        // deploy Blend Protocol dependencies
        let (backstop_id, backstop_client) = create_backstop(&e);
        let (emitter_id, emitter_client) = create_emitter(&e);
        let (pool_factory_id, _) = create_pool_factory(&e);

        std::println!("===================================== After Create With Backstop, Emitter, Pool Factory ===========================================");

        // deploy external dependencies
        let (lp, lp_client) = create_lp_pool(&e, &admin, &blnd_id, &mock_ousd_token_id.clone());

        std::println!("===================================== After Create Lp ===========================================");
        // initialize emitter
        // blnd_client.mint(&emitter_id, &(10_000_000 * SCALAR_7));
        blnd_client.set_admin(&emitter_id);
        emitter_client.initialize(&blnd_id, &backstop_id, &lp);

        std::println!("===================================== After Emitter Initialize ===========================================");

        // initialize backstop
        backstop_client.initialize(
            &lp,
            &emitter_id,
            &mock_ousd_token_id.clone(),
            &blnd_id,                        
            &pool_factory_id,
            &svec![
                &e,
                (admin.clone(), 10_000_000 * SCALAR_7),
                (frodo.clone(), 40_000_000 * SCALAR_7)
            ],
        );

        std::println!("===================================== After Backstop Initialize ===========================================");

        // initialize pool factory
        let pool_hash = e.deployer().upload_contract_wasm(POOL_WASM);
        let pool_init_meta = PoolInitMeta {
            backstop: backstop_id.clone(),
            pool_hash: pool_hash.clone(),
            blnd_id: blnd_id.clone(),
        };
        let pool_factory_client = PoolFactoryClient::new(&e, &pool_factory_id);
        pool_factory_client.initialize(&pool_init_meta);

        std::println!("===================================== After Pool Factory Initialize Initialize ===========================================");

        // drop tokens to bombadil
        backstop_client.drop();

        std::println!("===================================== After Backstop Drop Function ===========================================");

        let (_, mock_oracle_client) = create_mock_oracle(&e);

        std::println!("===================================== After Create Mock Oracle ===========================================");

        mock_oracle_client.set_data(
            &admin,
            &Asset::Other(Symbol::new(&e, "USD")),
            &svec![
                &e,
                Asset::Stellar(mock_ousd_token_id.clone()),
                Asset::Stellar(xlm_id.clone())
            ],
            &7,
            &300,
        );

        std::println!("===================================== After Oracle Set Data ===========================================");

        mock_oracle_client.set_price_stable(&svec![
            &e,
            1_0000000, // mock_ousd
            0_1000000,    // xlm
        ]);

        std::println!("===================================== After Oracle Set Price ===========================================");

        let (mock_treasury_id, mock_treasury_client) = create_mock_treasury(&e);
        let (mock_pegkeeper_id, mock_pegkeeper_client) = create_mock_pegkeeper(&e);

        // deploy Orbit dependencies
        let (bridge_oracle_id, bridge_oracle_client) = create_bridge_oracle(&e);
        // bridge_oracle_client.initialize(&treasury_factory_id, &bridge_oracle_id);

        std::println!("===================================== After Init Treasury, Pegkeeper, Receiver ===========================================");
        
        // // initialize treasury factory
        // let treasury_hash = e.deployer().upload_contract_wasm(MOCK_TREASURY_WASM);
        // let treasury_init_meta: TreasuryInitMeta = TreasuryInitMeta {
        //     treasury_hash: treasury_hash.clone(),
        //     pool_factory: pool_factory_id.clone(),
        // };
        // treasury_factory_client.initialize(&admin, &bridge_oracle_id, &treasury_init_meta);

        // std::println!("===================================== After Treasury Init ===========================================");

        // Initialize soroswap

        let (pair_factory_id, pair_factory_client) = create_pair_factory(&e);
        let pair_hash = e.deployer().upload_contract_wasm(PAIR_WASM);
        let (router_id, router_client) = create_router(&e);
        pair_factory_client.initialize(&admin, &pair_hash);
        router_client.initialize(&pair_factory_id);

        std::println!("===================================== Router Init ===========================================");

        // initialize oracle
        mock_treasury_client.initialize(&admin, &mock_ousd_token_id, &blnd_id, &router_id, &mock_ousd_token_id, &mock_pegkeeper_id);

        std::println!("===================================== After Mock Treasury Initialize ===========================================");

        mock_pegkeeper_client.initialize(&admin);

        std::println!("===================================== After Init Treasury, Pegkeeper ===========================================");

        let fixture = TestFixture {
            env: e,
            admin: admin,
            users: vec![frodo],
            emitter: emitter_client,
            backstop: backstop_client,
            pool_factory: pool_factory_client,
            pair_factory: pair_factory_client,
            router: router_client,
            oracle: mock_oracle_client,
            bridge_oracle: bridge_oracle_client,
            lp: lp_client,
            pools: vec![],
            pairs: vec![],
            tokens: vec![
                blnd_client,
                xlm_client,
                mock_ousd_token_client
            ],
            mock_treasury: mock_treasury_client,
            mock_pegkeeper: mock_pegkeeper_client,
        };
        fixture.jump(7 * 24 * 60 * 60);
        fixture
    }

    pub fn create_pool(&mut self, name: String, backstop_take_rate: u32, max_positions: u32) {
        std::println!("===================================== Create Pool Function ===========================================");
        // let from = self.tokens[TokenIndex::MockOusd].address.clone();
        // let to = self.tokens[TokenIndex::BLND].address.clone();
        let oracle_id = &self.bridge_oracle;
        
        std::println!("===================================== Create Pool Before ===========================================");

        let pool_id = self.pool_factory.deploy(
            &self.admin,
            &name,
            &BytesN::<32>::random(&self.env),
            &oracle_id.address.clone(),
            &backstop_take_rate,
            &max_positions,
        );

        std::println!("===================================== Create Pool After ===========================================");

        let ousd_id = &self.tokens[TokenIndex::MockOusd];
        let mock_treasury_client = &self.mock_treasury;

        // let treasury_id = self.treasury_factory.deploy(
        //     &BytesN::<32>::random(&self.env),
        //     &from,
        //     &FactoryAsset::Stellar(to.clone()),
        //     &pool_id
        // );

        // std::println!("===================================== Treasury Factory Deploy After ===========================================");

        ousd_id.set_admin(&mock_treasury_client.address);
        self.pools.push(PoolFixture {
            pool: PoolClient::new(&self.env, &pool_id),
            treasury: TreasuryClient::new(&self.env, &mock_treasury_client.address),
            reserves: HashMap::new(),
        });

        std::println!("===================================== Push Pool to Buffer ===========================================");
    }

    pub fn create_pair(&mut self, token_a: TokenIndex, token_b: TokenIndex) {
        let token_a_id = &self.tokens[token_a].address;
        let token_b_id = &self.tokens[token_b].address;
        let pair_id = self.pair_factory.create_pair(token_a_id, token_b_id);
        self.pairs.push(PairFixture {
            pair: PairClient::new(&self.env, &pair_id),
        });
    }

    pub fn create_pool_reserve(
        &mut self,
        pool_index: usize,
        asset_index: TokenIndex,
        reserve_config: &ReserveConfig,
    ) {
        std::println!("===================================== create_pool_reserve function ===========================================");
        let mut pool_fixture = self.pools.remove(pool_index);
        std::println!("===================================== remove pool index ===========================================");
        let token = &self.tokens[asset_index];
        pool_fixture
            .pool
            .queue_set_reserve(&token.address, reserve_config);
        std::println!("===================================== que set reserve ===========================================");
        let index = pool_fixture.pool.set_reserve(&token.address);
        pool_fixture.reserves.insert(asset_index, index);
        std::println!("===================================== insert reserve asset ===========================================");
        self.pools.insert(pool_index, pool_fixture);
        std::println!("===================================== insert pool index, pool fixture ===========================================");
    }

    /********** Contract Data Helpers **********/

    pub fn read_pool_config(&self, pool_index: usize) -> PoolConfig {
        let treasury_fixture = &self.pools[pool_index];
        self.env.as_contract(&treasury_fixture.pool.address, || {
            self.env
                .storage()
                .instance()
                .get(&Symbol::new(&self.env, "Config"))
                .unwrap()
        })
    }

    pub fn read_pool_emissions(&self, pool_index: usize) -> Map<u32, u64> {
        let treasury_fixture = &self.pools[pool_index];
        self.env.as_contract(&treasury_fixture.pool.address, || {
            self.env
                .storage()
                .persistent()
                .get(&Symbol::new(&self.env, "PoolEmis"))
                .unwrap()
        })
    }

    pub fn read_reserve_config(&self, pool_index: usize, asset_index: TokenIndex) -> ReserveConfig {
        let treasury_fixture = &self.pools[pool_index];
        let token = &self.tokens[asset_index];
        self.env.as_contract(&treasury_fixture.pool.address, || {
            let token_id = &token.address;
            self.env
                .storage()
                .persistent()
                .get(&PoolDataKey::ResConfig(token_id.clone()))
                .unwrap()
        })
    }

    pub fn read_reserve_data(&self, pool_index: usize, asset_index: TokenIndex) -> ReserveData {
        let treasury_fixture = &self.pools[pool_index];
        let token = &self.tokens[asset_index];
        self.env.as_contract(&treasury_fixture.pool.address, || {
            let token_id = &token.address;
            self.env
                .storage()
                .persistent()
                .get(&PoolDataKey::ResData(token_id.clone()))
                .unwrap()
        })
    }

    pub fn read_reserve_emissions(
        &self,
        pool_index: usize,
        asset_index: TokenIndex,
        token_type: u32,
    ) -> (ReserveEmissionsConfig, ReserveEmissionsData) {
        let treasury_fixture = &self.pools[pool_index];
        let reserve_index = treasury_fixture.reserves.get(&asset_index).unwrap();
        let res_emis_index = reserve_index * 2 + token_type;
        self.env.as_contract(&treasury_fixture.pool.address, || {
            let emis_config = self
                .env
                .storage()
                .persistent()
                .get(&PoolDataKey::EmisConfig(res_emis_index))
                .unwrap();
            let emis_data = self
                .env
                .storage()
                .persistent()
                .get(&PoolDataKey::EmisData(res_emis_index))
                .unwrap();
            (emis_config, emis_data)
        })
    }

    /********** Chain Helpers ***********/

    pub fn jump(&self, time: u64) {
        self.env.ledger().set(LedgerInfo {
            timestamp: self.env.ledger().timestamp().saturating_add(time),
            protocol_version: 20,
            sequence_number: self.env.ledger().sequence(),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 999999,
            min_persistent_entry_ttl: 999999,
            max_entry_ttl: 9999999,
        });
    }

    pub fn jump_with_sequence(&self, time: u64) {
        let blocks = time / 5;
        self.env.ledger().set(LedgerInfo {
            timestamp: self.env.ledger().timestamp().saturating_add(time),
            protocol_version: 20,
            sequence_number: self.env.ledger().sequence().saturating_add(blocks as u32),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 999999,
            min_persistent_entry_ttl: 999999,
            max_entry_ttl: 9999999,
        });
    }
}

use soroban_sdk::{log, Address, Env, vec, Val, Vec, IntoVal, Symbol};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
// use crate::storage::{self, LiquidateConfig, SwapConfig};
use crate::dependencies::{
    router::{Client as RouterClient},
    pool::{Client as PoolClient, Request},
};
use crate::storage;

// pub fn liquidate(e: &Env, context: &LiquidateConfig) {
//     log!(e, "================================= MockPegkeeper  liquidation Function ================================");
//     storage::extend_instance(e);

//     let fill_requests = vec![
//         &e,
//         Request {
//             request_type: 6 as u32,
//             address: context.auction_creator.clone(), // liquidationAuction
//             amount: context.auction_amount.clone(),
//         },
//         Request {
//             request_type: 5 as u32, // Repay
//             address: context.tokena.clone(),
//             amount: context.tokena_bid_amount.clone(),
//         },
//         Request {
//             request_type: 3 as u32, // Withdraw
//             address: context.tokenb.clone(),
//             amount: context.tokenb_lot_amount.clone(),
//         },
//     ];

//     let args: Vec<Val> = vec![
//         &e,
//         e.current_contract_address().into_val(e),
//         context.blend_pool.into_val(e),
//         context.tokena_bid_amount.into_val(e),
//     ];
//     e.authorize_as_current_contract(vec![
//         &e,
//         InvokerContractAuthEntry::Contract(SubContractInvocation {
//             context: ContractContext {
//                 contract: context.tokena.clone(),
//                 fn_name: Symbol::new(e, "transfer"),
//                 args: args.clone(),
//             },
//             sub_invocations: vec![&e],
//         })
//     ]);

//     log!(e, "================================= MockPegkeeper  Fill Request ================================");
//     PoolClient::new(e, &context.blend_pool).submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &fill_requests);

//     log!(e, "================================= MockPegkeeper  liquidation End ================================");
// }

// pub fn swap(e: &Env, context: &SwapConfig) {
//     log!(e, "================================= MockPegkeeper  Swap Function ================================");
//     storage::extend_instance(e);

//     let router = storage::get_router(e);
//     let router_client = RouterClient::new(e, &router);

//     let path = vec![
//         e,
//         context.tokena.clone(),
//         context.tokenb.clone(),
//     ];
//     let args: Vec<Val> = vec![
//         e,
//         e.current_contract_address().into_val(e),
//         context.pair.into_val(e),
//         context.tokena_amount.into_val(e),
//     ];
//     e.authorize_as_current_contract(vec![
//         e,
//         InvokerContractAuthEntry::Contract( SubContractInvocation {
//             context: ContractContext {
//                 contract: context.tokena.clone(),
//                 fn_name: Symbol::new(e, "transfer"),
//                 args: args.clone(),
//             },
//             sub_invocations: vec![e]
//         })
//     ]);
//     router_client.swap_exact_tokens_for_tokens(&context.tokena_amount, &context.tokenb_amount, &path, &e.current_contract_address(), &u64::MAX);
//     log!(e, "================================= MockPegkeeper  Swap End ================================");
// }

pub fn liquidate(e: &Env, auction_creator: Address, token_a: Address, token_a_bid_amount: i128, token_b: Address, token_b__lot_amount: i128, blend_pool: Address, liq_amount: i128) {
  log!(e, "================================= MockPegkeeper  liquidation Function ================================");
  storage::extend_instance(e);

  let fill_requests = vec![
      e,
      Request {
          request_type: 6 as u32,
          address: auction_creator.clone(), // liquidationAuction
          amount: liq_amount.clone(),
      },
      Request {
          request_type: 5 as u32, // Repay
          address: token_a.clone(),
          amount: token_a_bid_amount,
      },
      Request {
          request_type: 3 as u32, // Withdraw
          address: token_b.clone(),
          amount: token_b__lot_amount,
      },
  ];

  let args: Vec<Val> = vec![
      e,
      e.current_contract_address().into_val(e),
      blend_pool.into_val(e),
      token_a_bid_amount.into_val(e),
  ];
  e.authorize_as_current_contract(vec![
      e,
      InvokerContractAuthEntry::Contract(SubContractInvocation {
          context: ContractContext {
              contract: token_a.clone(),
              fn_name: Symbol::new(e, "transfer"),
              args: args.clone(),
          },
          sub_invocations: vec![e],
      })
  ]);

  log!(e, "================================= MockPegkeeper  Fill Request ================================");
  PoolClient::new(e, &blend_pool).submit(&e.current_contract_address(), &e.current_contract_address(), &e.current_contract_address(), &fill_requests);

  log!(e, "================================= MockPegkeeper  liquidation End ================================");
}

pub fn swap(e: &Env, pair: Address, token_a: Address, token_b: Address, amount_a: i128, amount_b: i128) {
  log!(e, "================================= MockPegkeeper  Swap Function ================================");
  storage::extend_instance(e);

  let router = storage::get_router(e);
  let router_client = RouterClient::new(e, &router);

  let path = vec![
      e,
      token_a.clone(),
      token_b.clone(),
  ];
  let args: Vec<Val> = vec![
      e,
      e.current_contract_address().into_val(e),
      pair.into_val(e),
      amount_a.into_val(e),
  ];
  e.authorize_as_current_contract(vec![
      e,
      InvokerContractAuthEntry::Contract( SubContractInvocation {
          context: ContractContext {
              contract: token_a.clone(),
              fn_name: Symbol::new(e, "transfer"),
              args: args.clone(),
          },
          sub_invocations: vec![e]
      })
  ]);
  router_client.swap_exact_tokens_for_tokens(&amount_a, &amount_b, &path, &e.current_contract_address(), &u64::MAX);
  log!(e, "================================= MockPegkeeper  Swap End ================================");
}
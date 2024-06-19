use soroban_sdk::{Address, Env, Symbol};
use soroban_sdk::unwrap::UnwrapOptimized;

pub(crate) const LEDGER_THRESHOLD_SHARED: u32 = 172800; // ~ 10 days
pub(crate) const LEDGER_BUMP_SHARED: u32 = 241920; // ~ 14 days

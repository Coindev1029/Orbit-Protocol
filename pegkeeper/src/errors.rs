use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PegkeeperError {
    /// not yet initialized
    NotInitialized = 101,

    /// already initialized
    AlreadyInitializedError = 102,
    
    /// not proper treasury for token
    NotProperTreasury = 103,

    /// uncorrect amount for loan
    UncorrectAmount = 104,

    /// unsufficient amount to repay
    InsufficientBalance = 105,

    /// failed to repay liabilities to blend
    RepayLiabilitiesFail = 106,
}
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
/// Error codes for the pool factory contract. Common errors are codes that match up with the built-in
/// dependencies error reporting. Treasury specific errors start at 2000.
pub enum TreasuryError {
    // Common Errors
    InternalError = 501,
    AlreadyInitializedError = 502,
    UnauthorizedError = 503,
    NegativeAmountError = 504,
    BalanceError = 505,
    InvalidAmount = 506,
    OverflowError = 507,
    FlashloanFailedError = 508,
    SupplyError = 509,
    FlashloanNotRepaid = 510
}
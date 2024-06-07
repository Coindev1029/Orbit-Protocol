use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
/// Error codes for the treasury factory contract. Common errors are codes that match up with the built-in
/// dependencies error reporting. Treasury factory specific errors start at 1300.
pub enum TreasuryFactoryError {
    // Common Errors
    InternalError = 1,
    AlreadyInitializedError = 3,
    NotInitializedError = 4,

    // Treasury Factory
    InvalidTreasuryInitArgs = 1300,
}
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
/// Error codes for the pool factory contract. Common errors are codes that match up with the built-in
/// contracts error reporting. Pool factory specific errors start at 1300.
pub enum TreasuryFactoryError {
    // Common Errors
    InternalError = 1,
    AlreadyInitializedError = 3,

    // Treasury Factory
    InvalidTreasuryInitArgs = 1300,
}
use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum MockReceiverError {
    /// not yet initialized
    NotInitialized = 101,

    /// already initialized
    AlreadyInitializedError = 102,
}
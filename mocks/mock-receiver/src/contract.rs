use soroban_sdk::{contract, contractclient, contractimpl, log, panic_with_error, vec, Address, Env, IntoVal, Symbol, Val, Vec};
use crate::{errors::MockReceiverError, storage};
#[contract]
pub struct MockReceiverContract;

#[contractclient(name="MockReceiverClient")]
pub trait MockReceiver {
    /// Initialize the treasury
    ///
    /// ### Arguments
    /// * `admin` - The Address for the admin
    /// * `maximum_duration` - The maximum_duration for swap transaction
    fn initialize(e: Env, admin: Address, maximum_duration: u64);

}

#[contractimpl]
impl MockReceiver for MockReceiverContract {
    fn initialize(e: Env, admin: Address, maximum_duration: u64) {
        storage::extend_instance(&e);

        if storage::is_init(&e) {
            panic_with_error!(&e, MockReceiverError::AlreadyInitializedError);
        }

        storage::set_admin(&e, &admin);
    }

}


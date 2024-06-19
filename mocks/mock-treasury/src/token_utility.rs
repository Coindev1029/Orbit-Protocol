use crate::{
    storage::get_token,
    errors::MockTreasuryError
};
use soroban_sdk::{token, Address, Env};

pub(crate) fn transfer(e: &Env, client: &token::Client, to: &Address, amount: &i128) {
    client.transfer(&e.current_contract_address(), to, amount);
}

pub(crate) fn get_token_client(e: &Env) -> token::Client {
    token::Client::new(
        e,
        &get_token(e), 
    )
}

pub(crate) fn transfer_in_treasury(env: &Env, client: &token::Client, from: &Address, amount: &i128) {
    client.transfer(from, &env.current_contract_address(), amount);
}

pub(crate) fn transfer_from_to_treasury(
    e: &Env,
    client: &token::Client,
    from: &Address,
    amount: &i128,
) -> Result<(), MockTreasuryError> {
    let res = client.try_transfer_from(
        &e.current_contract_address(),
        from,
        &e.current_contract_address(),
        amount,
    );

    if let Ok(Ok(_)) = res {
        Ok(())
    } else {
        Err(MockTreasuryError::FlashloanNotRepaid)
    }
}
pub(crate) fn try_repay(
    e: &Env,
    client: &token::Client,
    receiver_id: &Address,
    amount: i128,
    fee: i128,
) -> Result<(), MockTreasuryError> {
    transfer_from_to_treasury(e, client, receiver_id, &(amount + fee))?;

    Ok(())
}
use soroban_sdk::{Address, Env, token};


pub fn get_balance(e: &Env, contract_id: Address) -> i128 {
    // How many "contract_id" tokens does this contract holds?
    // We need to implement the token client
    token::TokenClient::new(e, &contract_id).balance(&e.current_contract_address())
}

pub fn transfer_amount(e: &Env, contract_id: Address, to: Address, amount: i128) {
    token::TokenClient::new(e, &contract_id).transfer(&e.current_contract_address(), &to, &amount);
}
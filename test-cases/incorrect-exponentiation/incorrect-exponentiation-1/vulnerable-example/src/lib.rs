#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Data,
}

#[contract]
pub struct IncorrectExponentiation;

#[contractimpl]
impl IncorrectExponentiation {

    pub fn set_data(e: Env, new_data: u128) {
        e.storage()
        .instance()
        .set::<DataKey, u128>(&DataKey::Data, &new_data);
    }

   
    pub fn exp_data_3(e: Env) -> u128 {
        let mut data = e.storage()
        .instance()
        .get::<DataKey, u128>(&DataKey::Data)
        .expect("Data not found");
        
        data ^= 3;
        data
    }

}

#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils, Address, Env};

    use crate::{IncorrectExponentiation, IncorrectExponentiationClient};

    #[test]
    fn simple_test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncorrectExponentiation);
    let client = IncorrectExponentiationClient::new(&env, &contract_id);
    env.mock_all_auths();
    let _user = <Address as testutils::Address>::generate(&env);

    client.set_data(&3_u128);
    
    assert_ne!(client.exp_data_3(), 27);
}
}

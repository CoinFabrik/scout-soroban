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

    pub fn init(e: Env){
        e.storage()
        .instance()
        .set::<DataKey, u128>(&DataKey::Data, &(255_u128.pow(2) - 1));
    }

    pub fn set_data(e: Env, new_data: u128) {
        e.storage()
        .instance()
        .set::<DataKey, u128>(&DataKey::Data, &new_data);
    }

    pub fn exp_data_3(e: Env) -> u128 {
        let data = e.storage()
        .instance()
        .get::<DataKey, u128>(&DataKey::Data)
        .expect("Data not found");

        data.pow(3)
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


    client.init();
    client.set_data(&10_u128);

    assert_eq!(client.exp_data_3(), 1000);
}
}



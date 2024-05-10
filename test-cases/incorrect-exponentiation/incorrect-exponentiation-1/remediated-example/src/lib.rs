#![no_std]

use soroban_sdk::{contract, contractimpl,contracterror, contracttype, Env};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Data,
}

// Agrega el atributo #[contracterror] a la definición de IEError
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum IEError {
    CouldntRetrieveData = 1,
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

    pub fn exp_data_3(e: Env) -> Result<u128, IEError> {
        let data:Option<u128> =  e.storage().instance().get(&DataKey::Data);
        match data 
        {
            Some(x) => return Ok(x.pow(3)),
            None =>return  Err(IEError::CouldntRetrieveData),
        }
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

    assert_eq!(client.exp_data_3(), 27);
}
}

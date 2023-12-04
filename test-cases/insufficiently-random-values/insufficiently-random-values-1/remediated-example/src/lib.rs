#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MaxValZero = 1,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn generate_random_value(env: Env, max_val: u64) -> Result<u64, Error> {
        if max_val == 0 {
            Err(Error::MaxValZero)
        } else {
            let val = env.prng().u64_in_range(0..max_val);
            Ok(val)
        }
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn random_value_sequence() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        // When
        let first_random_value = client.generate_random_value(&10);
        let second_random_value = client.generate_random_value(&10);
        let third_random_value = client.generate_random_value(&10);
        let fourth_random_value = client.generate_random_value(&10);
        let fifth_random_value = client.generate_random_value(&10);

        // Then
        assert_eq!(first_random_value, 6);
        assert_eq!(second_random_value, 5);
        assert_eq!(third_random_value, 8);
        assert_eq!(fourth_random_value, 8);
        assert_eq!(fifth_random_value, 4);
    }
}

#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct IntegerOverflowUnderflow;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OverflowError = 1,
    UnderflowError = 2,
}

#[contractimpl]
impl IntegerOverflowUnderflow {
    const VALUE: Symbol = symbol_short!("VALUE");

    pub fn initialize(env: Env, value: u32) {
        env.storage().temporary().set(&Self::VALUE, &value);
    }

    pub fn mul(env: Env, value: u32) -> Result<(), Error> {
        let current: u32 = env.storage().temporary().get(&Self::VALUE).unwrap_or(0);
        let new_value = match current.checked_mul(value) {
            Some(value) => value,
            None => return Err(Error::OverflowError),
        };
        env.storage().temporary().set(&Self::VALUE, &new_value);
        Ok(())
    }

    pub fn pow(env: Env, value: u32) -> Result<(), Error> {
        let current: u32 = env.storage().temporary().get(&Self::VALUE).unwrap_or(0);
        let new_value = match current.checked_pow(value) {
            Some(value) => value,
            None => return Err(Error::OverflowError),
        };
        env.storage().temporary().set(&Self::VALUE, &new_value);
        Ok(())
    }

    pub fn neg(env: Env) -> Result<(), Error> {
        let current: u32 = env.storage().temporary().get(&Self::VALUE).unwrap_or(0);
        let new_value = match current.checked_neg() {
            Some(value) => value,
            None => return Err(Error::UnderflowError),
        };
        env.storage().temporary().set(&Self::VALUE, &new_value);
        Ok(())
    }

    pub fn get(env: Env) -> u32 {
        env.storage().temporary().get(&Self::VALUE).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_initialize() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, IntegerOverflowUnderflow);
        let client = IntegerOverflowUnderflowClient::new(&env, &contract_id);

        // When
        client.initialize(&42);

        // Then
        assert_eq!(client.get(), 42);
    }

    #[test]
    fn test_mul_overflow() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, IntegerOverflowUnderflow);
        let client = IntegerOverflowUnderflowClient::new(&env, &contract_id);

        // When
        client.initialize(&u32::MAX);
        let result = client.try_mul(&2);

        // Then
        assert_eq!(result, Err(Ok(Error::OverflowError)));
    }

    #[test]
    fn test_pow_overflow() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, IntegerOverflowUnderflow);
        let client = IntegerOverflowUnderflowClient::new(&env, &contract_id);

        // When
        client.initialize(&2);
        let result = client.try_pow(&u32::MAX);

        // Then
        assert_eq!(result, Err(Ok(Error::OverflowError)));
    }

    #[test]
    fn test_neg() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, IntegerOverflowUnderflow);
        let client = IntegerOverflowUnderflowClient::new(&env, &contract_id);

        // When
        client.initialize(&u32::MAX);
        let result = client.try_neg();

        // Then
        assert_eq!(result, Err(Ok(Error::UnderflowError)));
    }
}

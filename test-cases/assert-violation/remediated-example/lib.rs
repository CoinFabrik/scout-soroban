#![no_std]

use soroban_sdk::{contracterror, contract, contractimpl, contracttype, Env, Symbol, symbol_short}; 

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum AVError {
    GreaterThan10 = 1,
}

#[derive(Debug, Clone)]
#[contracttype]
pub struct State {
    value: u128,
}

const STATE: Symbol = symbol_short!("STATE"); 
#[contract]
pub struct AssertViolation; 

#[contractimpl]
impl AssertViolation {
    pub fn init(env: Env, init_value: u128) -> State {
        let state = State {
            value: init_value
        };

        env.storage().instance().set(&STATE, &state);
        state

    }

    pub fn assert_if_greater_than_10(_env: Env, value: u128) -> Result<bool, AVError> {
        if value <= 10 {
            Ok(true)
        } else {
            Err(AVError::GreaterThan10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn does_not_revert_if_greater() {
        let env = Env::default(); 
        let contract = AssertViolationClient::new(&env, &env.register_contract(None, AssertViolation{})); 
        assert_eq!(contract.assert_if_greater_than_10(&5), true);
    }

    #[test]
    #[should_panic(expected = 1)] // The custom error number is 1
    fn reverts_if_greater() {
        let env = Env::default(); 
        let contract = AssertViolationClient::new(&env, &env.register_contract(None, AssertViolation{})); 
        contract.assert_if_greater_than_10(&11);
    }

}
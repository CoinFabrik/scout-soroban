#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl};

#[contract]
pub struct UnnecessaryLintAllow;

#[contracterror]
#[derive(Copy, Clone)]
pub enum AssertError {
    GreaterThan10 = 1,
}

#[contractimpl]
impl UnnecessaryLintAllow {
    pub fn assert_if_greater_than_10(value: u128) -> Result<bool, AssertError> {
        if value <= 10 {
            Ok(true)
        } else {
            Err(AssertError::GreaterThan10)
        }
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::Env;

    use super::*;
    #[test]
    fn does_not_revert_if_greater() {
        let env = Env::default();
        let contract = UnnecessaryLintAllowClient::new(
            &env,
            &env.register_contract(None, UnnecessaryLintAllow {}),
        );
        assert!(contract.assert_if_greater_than_10(&5));
    }

    #[test]
    #[should_panic(expected = "1")] // The custom error number is 1
    fn reverts_if_greater() {
        let env = Env::default();
        let contract = UnnecessaryLintAllowClient::new(
            &env,
            &env.register_contract(None, UnnecessaryLintAllow {}),
        );
        contract.assert_if_greater_than_10(&11);
    }
}

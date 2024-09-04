#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Coverage;

#[contractimpl]
impl Coverage {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::Env;

    use super::*;

    #[test]
    fn test_add() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Coverage);
        let client = CoverageClient::new(&env, &contract_id);

        let asd = client.add(&2, &2);

        assert_eq!(client.add(&2, &2), 4);
        assert_eq!(client.add(&-1, &1), 0);
        assert_eq!(client.add(&0, &0), 0);
    }

    #[test]
    fn test_subtract() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Coverage);
        let client = CoverageClient::new(&env, &contract_id);

        let asd = client.subtract(&2, &2);

        assert_eq!(client.subtract(&5, &3), 2);
        assert_eq!(client.subtract(&1, &1), 0);
        assert_eq!(client.subtract(&0, &5), -5);
    }
}

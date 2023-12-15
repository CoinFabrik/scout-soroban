#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct SetContractStorage;

#[contractimpl]
impl SetContractStorage {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env, user: Address) -> u32 {
        user.require_auth();
        let storage = env.storage().persistent();
        let mut count: u32 = storage.get(&COUNTER).unwrap_or(0);
        count += 1;
        storage.set(&COUNTER, &count);
        storage.extend_ttl(&COUNTER, 100, 100);
        count
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils, Address, Env};

    use crate::{SetContractStorage, SetContractStorageClient};

    #[test]
    fn increment() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, SetContractStorage);
        let client = SetContractStorageClient::new(&env, &contract_id);
        env.mock_all_auths();
        let user = <Address as testutils::Address>::generate(&env);

        // When
        let first_increment = client.increment(&user);
        let second_increment = client.increment(&user);
        let third_increment = client.increment(&user);

        // Then
        assert_eq!(first_increment, 1);
        assert_eq!(second_increment, 2);
        assert_eq!(third_increment, 3);
    }
}

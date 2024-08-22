#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Symbol};

#[contract]
pub struct StringStorage;

#[contractimpl]
impl StringStorage {
    pub fn store_string(e: Env, data: String) {
        e.storage()
            .instance()
            .set(&Symbol::new(&e, "string_data"), &data);
    }

    pub fn get_string(e: Env) -> String {
        e.storage()
            .instance()
            .get(&Symbol::new(&e, "string_data"))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_string_storage() {
        // Given
        let env = Env::default();
        let contract_id = env.register_contract(None, StringStorage);
        let client = StringStorageClient::new(&env, &contract_id);

        // When
        let test_string = String::from_str(&env, "Hello, Soroban!");
        client.store_string(&test_string);

        // Then
        let retrieved_string = client.get_string();
        assert_eq!(test_string, retrieved_string);
    }
}

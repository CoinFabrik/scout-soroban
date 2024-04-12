use crate::{IncrementorContract, IncrementorContractClient};
use soroban_sdk::Env;

#[test]
fn increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementorContract);
    let client = IncrementorContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}
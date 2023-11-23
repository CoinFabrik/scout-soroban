extern crate std;

use std::dbg;

use soroban_sdk::{
    symbol_short,
    testutils::{Ledger, LedgerInfo},
    vec, Bytes, Env,
};

use crate::{Contract, ContractClient};

#[test]
fn random_value_sequence() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    assert_eq!(client.generate_random_value(&10), 6);
    assert_eq!(client.generate_random_value(&10), 5);
    assert_eq!(client.generate_random_value(&10), 8);
    assert_eq!(client.generate_random_value(&10), 8);
    assert_eq!(client.generate_random_value(&10), 4);
}

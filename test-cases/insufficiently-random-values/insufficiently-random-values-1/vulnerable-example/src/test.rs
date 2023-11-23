extern crate std;

use soroban_sdk::{
    testutils::{Ledger, LedgerInfo},
    Env,
};

use crate::{Contract, ContractClient};

#[test]
fn random_value_sequence() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let mut ledger = LedgerInfo::default();
    ledger.sequence_number = 0;
    env.ledger().set(ledger.clone());

    let mut random = client.generate_random_value_sequence(&10);
    assert_eq!(random, 0);

    ledger.sequence_number = 1;
    env.ledger().set(ledger.clone());

    random = client.generate_random_value_sequence(&10);
    assert_eq!(random, 1);

    ledger.sequence_number = 11;
    env.ledger().set(ledger.clone());

    random = client.generate_random_value_sequence(&10);
    assert_eq!(random, 1);
}
#[test]
fn random_value_timestamp() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let mut ledger = LedgerInfo::default();
    ledger.timestamp = 0;
    env.ledger().set(ledger.clone());

    let mut random = client.generate_random_value_timestamp(&10);
    assert_eq!(random, 0);

    ledger.timestamp = 1;
    env.ledger().set(ledger.clone());

    random = client.generate_random_value_timestamp(&10);
    assert_eq!(random, 1);

    ledger.timestamp = 11;
    env.ledger().set(ledger.clone());

    random = client.generate_random_value_timestamp(&10);
    assert_eq!(random, 1);
}

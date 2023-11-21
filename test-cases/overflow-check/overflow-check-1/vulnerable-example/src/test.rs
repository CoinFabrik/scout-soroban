extern crate std;

use std::dbg;

use soroban_sdk::{symbol_short, vec, Env};

use crate::{Contract, ContractClient};

#[test]
fn hello() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let words = client.try_hello(&symbol_short!("Dev"));
    dbg!(words);
    /*assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );*/
}

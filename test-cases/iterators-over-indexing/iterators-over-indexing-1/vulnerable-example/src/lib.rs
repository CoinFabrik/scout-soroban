#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    Env,
    vec,
    Vec,
};

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Data,
}

#[contract]
pub struct IteratorsOverIndexingVulnerableContract;

#[contractimpl]
impl IteratorsOverIndexingVulnerableContract {
    pub fn init(e: Env){
        e.storage().instance().set::<DataKey, Vec<i32>>(&DataKey::Data, &vec![&e, 1_i32, 2_i32, 3_i32, 4_i32]);
    }

    pub fn sum(e: Env) -> i32{
        let mut ret = 0;
        let vec = e.storage().instance().get::<DataKey, Vec<i32>>(&DataKey::Data).unwrap();
        for i in 0..4{
            ret += vec.get(i).unwrap();
        }
        ret
    }
}

#[test]
fn simple_test(){
    let e = Env::default();
    e.mock_all_auths();
    let client = IteratorsOverIndexingVulnerableContractClient::new(&e, &e.register_contract(None, IteratorsOverIndexingVulnerableContract {}));
    client.init();
    assert_eq!(client.sum(), 10);
}

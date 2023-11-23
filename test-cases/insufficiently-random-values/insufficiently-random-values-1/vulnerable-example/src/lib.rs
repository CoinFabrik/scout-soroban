#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MaxValZero = 1,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn generate_random_value_timestamp(env: Env, max_val: u64) -> Result<u64, Error> {
        if max_val == 0 {
            Err(Error::MaxValZero)
        } else {
            let val = env.ledger().timestamp() % max_val;
            Ok(val)
        }
    }
    pub fn generate_random_value_sequence(env: Env, max_val: u32) -> Result<u32, Error> {
        if max_val == 0 {
            Err(Error::MaxValZero)
        } else {
            let val = env.ledger().sequence() % max_val;
            Ok(val)
        }
    }
}

#[cfg(test)]
mod test;

#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, vec, Env, Symbol, Vec};

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
    pub fn generate_random_value(env: Env, max_val: u64) -> Result<u64, Error> {
        if max_val == 0 {
            Err(Error::MaxValZero)
        } else {
            let val = env.prng().u64_in_range(0..max_val);
            Ok(val)
        }
    }
}

#[cfg(test)]
mod test;

#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct UnsafeUnwrap;

#[contractimpl]
impl UnsafeUnwrap {
    pub fn unwrap_an_empty_thing() -> u64 {

        let a_thing = None;
        let result = a_thing.unwrap_or(0);

        result
    }

    pub fn unwrap_a_thing(n: u64) -> u64 {

        let a_thing = Some(n);
        let result = a_thing.unwrap_or(0);

        result
    }

}

#[cfg(test)]
mod tests {
    use crate::UnsafeUnwrap;

    #[test]
    fn unwrap_an_empty_thing_panics() {
        UnsafeUnwrap::unwrap_an_empty_thing();
    }

    #[test]
    fn unwrap_a_thing_panics() {
        UnsafeUnwrap::unwrap_a_thing(100);
    }
}

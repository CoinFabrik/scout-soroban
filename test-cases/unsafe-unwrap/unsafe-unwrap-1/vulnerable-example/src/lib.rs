#![no_std]
use soroban_sdk::{contract, contractimpl, contracterror};

#[contract]
pub struct UnsafeUnwrap;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    CustomError = 1,
}

#[contractimpl]
impl UnsafeUnwrap {

    pub fn unwrap_a_thing(n: u64) -> u64 {

        let a_thing = Self::return_a_result(n);

        a_thing.unwrap()
    }
    pub fn return_a_result(n: u64) -> Result<u64, Error> {

        if n == 0 {
            return Err(Error::CustomError);
        }

        Ok(n)
    }

}

#[cfg(test)]
mod tests {
    use crate::UnsafeUnwrap;

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn unwrap_an_empty_thing_panics() {
        UnsafeUnwrap::unwrap_an_empty_thing();
    }

    #[test]
    fn unwrap_a_thing_panics() {
        UnsafeUnwrap::unwrap_a_thing(100);
    }
}

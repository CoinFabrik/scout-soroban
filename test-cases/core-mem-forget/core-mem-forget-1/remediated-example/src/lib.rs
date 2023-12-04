#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype};

#[contract]
pub struct CoreMemForget;

#[contracttype]
#[derive(Eq, PartialEq)]
pub struct WithoutCopy {
    pub a: u64,
    pub b: u64,
}

#[contractimpl]
impl CoreMemForget {
    pub fn forget_something(n: WithoutCopy) -> u64 {
        let _ = n;
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_forget_something() {
        // Given
        let test_value: WithoutCopy = WithoutCopy { a: 80, b: 60 };

        // When
        let result = CoreMemForget::forget_something(test_value);

        // Then
        assert_eq!(result, 0);
    }
}

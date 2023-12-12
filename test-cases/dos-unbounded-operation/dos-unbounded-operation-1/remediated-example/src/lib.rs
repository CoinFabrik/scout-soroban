#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct DosUnboundedOperation;

const FIXED_COUNT: u64 = 500;

#[contractimpl]
impl DosUnboundedOperation {
    pub fn safe_loop_with_const() -> u64 {
        let mut sum = 0;
        for i in 0..FIXED_COUNT {
            sum += i;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::DosUnboundedOperation;

    #[test]
    fn test_unrestricted_for_loop() {
        // Given
        // ..

        // When
        let count = DosUnboundedOperation::safe_loop_with_const();

        // Then
        assert_eq!(count, 124750);
    }
}

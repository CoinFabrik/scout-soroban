#![no_std]
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct DosUnboundedOperation;

#[contractimpl]
impl DosUnboundedOperation {
    pub fn unrestricted_loop(for_loop_count: u64) -> u64 {
        let mut count = 0;
        for i in 0..for_loop_count {
            count += i;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::DosUnboundedOperation;

    #[test]
    fn test_for_loop() {
        // Given
        // ..

        // When
        let for_loop_count = 1000;
        let count = DosUnboundedOperation::unrestricted_loop(for_loop_count);

        // Then
        assert_eq!(count, 499500);
    }
}

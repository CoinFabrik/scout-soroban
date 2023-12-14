#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype};

#[contract]
pub struct DosUnboundedOperation;

#[contracttype]
pub struct KnownData {
    fixed_value: u64,
}

#[contractimpl]
impl DosUnboundedOperation {
    pub fn safe_loop_with_struct() -> u64 {
        let mut sum = 0;
        let known_data = KnownData { fixed_value: 1000 };
        for i in 0..known_data.fixed_value {
            sum += i;
        }
        sum
    }
}

#[cfg(test)]
mod tests {

    use crate::DosUnboundedOperation;

    #[test]
    fn test_for_loop() {
        // Given
        // ...

        // When
        let count = DosUnboundedOperation::safe_loop_with_vector();

        // Then
        assert_eq!(count, 499500);
    }
}

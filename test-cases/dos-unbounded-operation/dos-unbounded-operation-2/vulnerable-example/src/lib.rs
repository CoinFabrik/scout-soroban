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
    pub fn unsafe_loop_with_struct(unknown_data: KnownData) -> u64 {
        let mut sum = 0;
        for i in 0..unknown_data.fixed_value {
            sum += i;
        }
        sum
    }
}

#[cfg(test)]
mod tests {

    use crate::{DosUnboundedOperation, KnownData};

    #[test]
    fn test_for_loop() {
        // Given
        // ...

        // When
        let unknown_data = KnownData { fixed_value: 1000 };
        let count = DosUnboundedOperation::unsafe_loop_with_struct(unknown_data);

        // Then
        assert_eq!(count, 499500);
    }
}

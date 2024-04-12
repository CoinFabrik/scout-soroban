# Unused return enum

### What it does

It warns if a function that returns a Result type does not return the Result enum variant (Ok/Err).

### Why is this bad?

If any of the variants (Ok/Err) is not used, the code could be simplified or it could imply a bug.

### Known problems

If definitions of `Err()` and/or `Ok()` are in the code but do not flow to the return value due to the definition of a variable or because they are defined in a dead code block, the warning will not be shown. If the definitions are made in an auxiliary method, the warning will be shown, resulting in a false positive.

### Example

Instead of using:

```rust
#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl};

#[contract]
pub struct UnusedReturnEnum;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// An overflow was produced.
    Overflow = 1,
}

#[contractimpl]
impl UnusedReturnEnum {
    /// Returns the percentage difference between two values.
    pub fn get_percentage_difference(balance1: u128, balance2: u128) -> Result<u128, Error> {
        let absolute_difference = balance1.abs_diff(balance2);
        let sum = balance1 + balance2;

        match 100u128.checked_mul(absolute_difference / sum) {
            Some(result) => result,
            None => panic!("Overflow"),
        };

        Err(Error::Overflow)
    }
}
```

Use this:

```rust
#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, testutils::arbitrary::arbitrary::Result};

#[contract]
pub struct UnusedReturnEnum;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// An overflow was produced.
    Overflow = 1,
}

#[contractimpl]
impl UnusedReturnEnum {
    /// Returns the percentage difference between two values.
    pub fn get_percentage_difference(balance1: u128, balance2: u128) -> Result<u128, Error> {
        let absolute_difference = balance1.abs_diff(balance2);
        let sum = balance1 + balance2;

        match 100u128.checked_mul(absolute_difference / sum) {
            Some(result) => Ok(result),
            None => Err(Error::Overflow),
        }
    }
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unused-return-enum).
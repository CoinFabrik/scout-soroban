# Assert  violation

### What it does​

Checks for `assert!` macro usage.

### Why is this bad?​

The `assert!` macro can cause the contract to panic.

### Example​

```rust
pub fn assert_if_greater_than_10(_env: Env, value: u128) -> bool {
       assert!(value <= 10, "value should be less than 10");
       true
   }

```
Use instead:

```rust
pub fn assert_if_greater_than_10(_env: Env, value: u128) -> Result<bool, AVError> {
       if value <= 10 {
           Ok(true)
       } else {
           Err(AVError::GreaterThan10)
       }
   }
```
### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/assert-violation).

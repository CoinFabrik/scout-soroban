# Incorrect Exponentiation

### What it does

Warns about `^` being a `bit XOR` operation instead of an exponentiation.  

### Why is this bad?

It can introduce unexpected behaviour in the smart contract.

#### More info

- https://doc.rust-lang.org/std/ops/trait.BitXor.html#tymethod.bitxor

### Example

```rust
    pub fn exp_data_3(e: Env) -> u128 {
        let mut data = e.storage()
        .instance()
        .get::<DataKey, u128>(&DataKey::Data)
        .expect("Data not found");
        data = data ^ 3;
        return data;
    }
```
Use instead:

```rust
     pub fn exp_data_3(e: Env) -> u128 {
        let data = e.storage()
        .instance()
        .get::<DataKey, u128>(&DataKey::Data)
        .expect("Data not found");
        return data.pow(3);
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/incorrect-exponentiation).

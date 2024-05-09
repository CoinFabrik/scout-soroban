# Incorrect Exponentiation

### What it does

Warns about `^` being a `bit XOR` operation instead of an exponentiation.  

### Why is this bad?

It can introduce unexpected behaviour in the smart contract.

#### More info

- https://doc.rust-lang.org/std/ops/trait.BitXor.html#tymethod.bitxor

### Example

```rust
    pub fn init(e: Env){
        e.storage()
            .instance()
            .set::<DataKey, u128>(&DataKey::Data, &((255_u128 ^ 2) - 1));
    }
```
Use instead:

```rust
     pub fn init(e: Env) {
        e.storage()
            .instance()
            .set::<DataKey, u128>(&DataKey::Data, &(255_u128.pow(2) - 1));
    }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/incorrect-exponentiation).

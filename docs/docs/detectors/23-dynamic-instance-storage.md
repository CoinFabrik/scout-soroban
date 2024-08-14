# Dynamic instance storage

## Description 

- Category: `Authorization`
- Severity: `Critical`
- Detectors: [`dynamic-instance-storage`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dynamic-instance-storage)
- Test Cases: [`dynamic-instance-storage-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dynamic-instance-storage/dynamic-instance-storage-1) [`dynamic-instance-storage-2`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dynamic-instance-storage/dynamic-instance-storage-2) [`dynamic-instance-storage-3`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dynamic-instance-storage/dynamic-instance-storage-3)

In Rust, it is very useful to use `storage.instance()` to store data that is shared among all users of the contract (e.g., a token administrator). However, using this macro with dynamic variables (such as vectors, maps, etc.) is not recommended.

## Why is this bad? 

Using dynamic values with `storage.instance()` can cause excessive storage use and may risk DoS attacks on the contract.

## Issue example 

Consider the following `Soroban` contract:

```rust

  pub fn store_vector(e: Env, data: Vec<i32>) {
        e.storage()
            .instance()
            .set(&Symbol::new(&e, "vector_data"), &data);
    }

```
In this example, the function is storing a vector using `storage.instance()`.

The code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dynamic-instance-storage/dynamic-instance-storage-1/vulnerable-example).

## Remediated example

Consider the following `Soroban` contract:

```rust
pub fn store_vector(e: Env, data: Vec<i32>) {
        e.storage()
            .persistent()
            .set(&Symbol::new(&e, "vector_data"), &data);
    } 
```

Instead of using `storage.instance()` to store a vector, you could use `storage.persistent()` to avoid memory issues or the risk of attacks. 

The remediated code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dynamic-instance-storage/dynamic-instance-storage-1/remediated-example).


## How is it detected?

Checks the usage of `storage().instance()` with dynamic types.

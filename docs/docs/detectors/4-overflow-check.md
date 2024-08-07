# Overflow check

## Description 

- Category: `Arithmetic`
- Severity: `Critical`
- Detector: [`overflow-check`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/overflow-check)
- Test Cases: [`overflow-check-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/overflow-check-1) 

In Rust, if the 'overflow-check' dependency is disabled in the Cargo.toml file, operations with integer overflow issues cannot be adjusted, leading to potential problems with the results obtained

## Why is this bad? 

Using arithmetic operations with integer overflow without regulation leads to wrong results, which can cause issues with other operations.

## Issue example 

Consider the following Cargo.toml, in a Soroban contract:

```rust
[profile.release]
overflow-checks = false
```
Problems can arise if `overflow-checks` is disabled.

The code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/overflow-check/overflow-check-1/vulnerable-example).


## Remediated example

```rust

 [profile.release]
  overflow-checks = true
        
```

The remediated code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/overflow-check/overflow-check-1/remediated-example).

## How is it detected?

Checks that `overflow-checks` is enabled in the `[profile.release]` section of the `Cargo.toml`.



    

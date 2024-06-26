# Unsafe expect

## Description

- Vulnerability Category: `Validations and error handling`
- Vulnerability Severity: `Minor`
- Detectors: [`unsafe-expect`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-expect)
- Test Cases: [`unsafe-expect-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-expect/unsafe-expect-1)

In Rust, the `expect` method is often used for error handling. It returns the contained `Ok` value for a `Result` or `Some` value for an `Option`. If an error occurs, it calls `panic!` with a provided error message.

The usage of `expect` can lead to a panic and crash the program, which is not desired behavior in most cases, especially for a smart contract.

## Exploit Scenario

Consider the following `Soroban` contract:

```rust

 pub fn balance_of(env: Env, owner: Address) -> i128 {
    let state = Self::get_state(env);
    state.balances.get(owner).expect("could not get balance")
}
  
```

In this contract, the `balance_of` function uses the expect method to retrieve the balance of an account. If there is no entry for this account in the balances mapping, the contract will panic and halt execution, which could be exploited maliciously to disrupt the contract's operation.

The vulnerable code example can be found [`here`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-expect/unsafe-expect-1/vulnerable-example).

## Remediation

Instead of using `expect`, use a safer method for error handling. In this case, if there is no entry for an account in the `balances` mapping, return a default value (like `0`).

```rust

pub fn balance_of(env: Env, owner: Address) -> i128 {
    let state = Self::get_state(env);
    state.balances.get(owner).unwrap_or(0)
}

```

The remediated code example can be found [`here`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-expect/unsafe-expect-1/remediated-example).

## References

[Rust documentation: `expect`](https://doc.rust-lang.org/std/option/enum.Option.html#method.expect)

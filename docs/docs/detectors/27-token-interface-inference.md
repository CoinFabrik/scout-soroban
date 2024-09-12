# Token interface inference

## Description 

- Category: `Best practices`
- Severity: `Enhancement`
- Detector: [`token-interface-inference`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/token-interface-inference)
- Test Cases: [`token-interface-inference-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-inference/token-interface-inference-1)

In Rust, the use of tokens requires strictly following a set of specifications. If any of these are not met, the contract will have errors and fail. Therefore, the use of the `soroban_sdk::token::TokenInterface` trait is really useful because it warns if any of the specifications are not being fulfilled.

## Why is this bad? 

Not using the `soroban_sdk::token::TokenInterface` trait makes it more difficult to comply with the token interface.

## Issue example 

Consider the following `Soroban` contract:

```rust

 #[contract]
pub struct TokenInterfaceInference;

#[contractimpl]
impl TokenInterfaceInference {
    pub fn initialize(
        env: Env,
        admin: Address,
        decimals: u32,
        name: String,
        symbol: String,
    ) -> Result<(), TIIError> {
        let current_token_metadata: Option<TokenMetadata> =
            env.storage().instance().get(&DataKey::TokenMetadata);
        if current_token_metadata.is_some() {
            return Err(TIIError::AlreadyInitialized);
        } else {
            env.storage().instance().set(
                &DataKey::TokenMetadata,
                &TokenMetadata {
                    decimals,
                    name,
                    symbol,
                    admin,
                },
            );
        }

        Ok(())
    }

```


In this example, the trait `soroban_sdk::token::TokenInterface` is not implemented; instead, a contract named "TokenInterfaceInference" is implemented.

The code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-inference/token-interface-inference-1/vulnerable-example).


## Remediated example

Consider the following `Soroban` contract:

```rust

 #[contractimpl]
impl token::TokenInterface for TokenInterfaceEvents {
    fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance = Self::get_allowance(env.clone(), from, spender);
        if allowance.expiration_ledger < env.ledger().sequence() {
            0
        } else {
            allowance.amount
        }
    }
        
```

In this example, the trait `soroban_sdk::token::TokenInterface` is implemented on a contract called "TokenInterfaceEvents".

The remediated code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/token-interface-inference/token-interface-inference-1/remediated-example).

## How is it detected?

Checks if token handling is being implemented in the contract and warns if the trait `soroban_sdk::token::TokenInterface` is not being used.










    

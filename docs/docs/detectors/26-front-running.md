# Front running

## Description 

- Category: `MEV`
- Severity: `Warning`
- Detector: [`front-running`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/front-running)
- Test Cases: [`front-running-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/front-running/front-running-1)

In Rust, when making a transfer and the amount to be sent is calculated rather than passed as a parameter, it would be ideal to pass another parameter that sets a minimum threshold for the amount to be transferred. This way, front-running can be avoided.

## Why is this bad? 

The front running attack results in a loss of funds for the victim of the attack and disrupts the normal functioning of the contract.

## Issue example 

Consider the following `Soroban` contract:

```rust

 pub fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        let transfer_amount = get_conversion_price(amount);
        TokenClient::new(&e, &get_token(&e)).transfer(&from, &to, &transfer_amount);
    }

```

In this example, the transfer function does not include a parameter indicating a minimum amount to be transferred.

The code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/front-running/front-running-1/vulnerable-example).


## Remediated example

Consider the following `Soroban` contract:

```rust

 pub fn transfer(e: Env, from: Address, to: Address, amount: i128, min_amount: i128) {
        let transfer_amount = get_conversion_price(amount);
        assert!(transfer_amount >= min_amount, "Insufficient amount");
        TokenClient::new(&e, &get_token(&e)).transfer(&from, &to, &transfer_amount);
    }

```
In this example, the transfer function includes a parameter indicating a minimum amount to be transferred.

The remediated example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/front-running/front-running-1/remediated-example).

## How is it detected?

Checks if there is a comparison between the amount to be transferred and another value.



    

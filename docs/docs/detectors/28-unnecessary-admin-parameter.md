# Unnecessary admin parameter

## Description 

- Category: `Access control`
- Severity: `Medium`
- Detector: [`unnecessary-admin-parameter`](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unnecessary-admin-parameter)
- Test Cases: [`unnecessary-admin-parameter-1`](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unnecessary-admin-parameter/unnecessary-admin-parameter-1)

In rust, a bad practice would be passing variables with names similar to "admin" as parameters that are used in `require_auth` calls within the functions of the contract. Ideally, the admin information should be retrieved directly from storage to avoid security issues in the contract.

## Why is this bad? 

Passing admin information as parameters can cause security issues, as the variable could be manipulated, resulting in incorrect authentication verification.

## Issue example 

Consider the following `Soroban` contract:

```rust

  pub fn set_admin(env: Env, new_admin: Address, admin: Address) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

```

In this example the admin is being passed as a parameter and then validated with `require_auth`.

The code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unnecessary-admin-parameter/unnecessary-admin-parameter-1/vulnerable-example).


## Remediated example

Consider the following `Soroban` contract:

```rust

 pub fn set_admin(env: Env, new_admin: Address) {
        // Initialize has already set the admin, so we can retrieve it directly.
        let current_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        current_admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }
        
```

In this example, the admin information is retrieved directly from storage and then validated with `require_auth`.

The remediated code example can be found [here](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unnecessary-admin-parameter/unnecessary-admin-parameter-1/remediated-example).

## How is it detected?

Checks the use of function parameters with names similar to "admin."



    

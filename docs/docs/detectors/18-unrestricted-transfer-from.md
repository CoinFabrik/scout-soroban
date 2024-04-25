# Unrestricted Transfer From

### What it does

It warns you if a `transfer_from` function is called with a user-defined parameter in the `from` field.

### Why is this bad?

An user Alice can approve a contract to spend their tokens. An user Bob can call that contract, use that allowance to send themselves Alice's tokens. 


### Example


```rust
pub fn deposit(env: Env, from: Address) -> Result<(), UTFError> {
    let mut state: State = Self::get_state(env.clone())?;
    state.buyer.require_auth();
    if state.status != Status::Created {
        return Err(UTFError::StatusMustBeCreated);
    }
    let token_client = token::Client::new(&env, &state.token);
    token_client.transfer_from(
        &env.current_contract_address(),
        &from,
        &env.current_contract_address(),
        &state.amount,
    );
    state.status = Status::Locked;
    env.storage().instance().set(&STATE, &state);
    Ok(())
}
```


Use instead:

```rust
  pub fn deposit(env: Env) -> Result<(), UTFError> {
      let mut state: State = Self::get_state(env.clone())?;
      state.buyer.require_auth();
      if state.status != Status::Created {
          return Err(UTFError::StatusMustBeCreated);
      }
      let token_client = token::Client::new(&env, &state.token);
      token_client.transfer_from(
          &env.current_contract_address(),
          &state.buyer,
          &env.current_contract_address(),
          &state.amount,
      );
      state.status = Status::Locked;
      env.storage().instance().set(&STATE, &state);
      Ok(())
  }
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unrestricted-transfer-from).

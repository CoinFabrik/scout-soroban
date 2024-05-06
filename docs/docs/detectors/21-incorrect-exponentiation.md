# Zero or test address

### What it does
Checks whether the zero address is being inputed to a function without validation.

### Why is this bad?
Because the private key for the zero address is known, anyone could take ownership of the contract.

### Example

```rust
pub fn set(e: Env, admin: Address, data: i32) -> Result<(), Error> {
    if !ZeroAddressContract::ensure_is_admin(&e, admin)? {
        return Err(Error::NotAdmin);
    }
    e.storage().persistent().set(&DataKey::Data, &data);
    Ok(())
}
```


Use instead:
```rust
pub fn set(e: Env, admin: Address, data: i32) -> Result<(), Error> {
    if admin
        == Address::from_string(&String::from_bytes(
            &e,
            b"GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF",
        ))
    {
        return Err(Error::InvalidNewAdmin);
    }
    if !ZeroAddressContract::ensure_is_admin(&e, admin)? {
        return Err(Error::NotAdmin);
    }
    e.storage().persistent().set(&DataKey::Data, &data);
    Ok(())
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/zero-or-test-address).

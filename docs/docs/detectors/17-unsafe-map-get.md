# Unsafe map get

### What it does

This detector identifies instances where unsafe methods like `get`, `get_unchecked`, and `try_get_unchecked` are used on `Map` objects in Soroban smart contracts.

### Why is this bad?

These methods are risky because they can lead to panics if the key does not exist in the map. Using these methods without proper checks increases the risk of runtime errors that can disrupt the execution of the smart contract and potentially lead to unexpected behavior or denial of service.

### Example

```rust
pub fn get_from_map(env: Env) -> Option<i32> {
    let map: Map<Val, Val> = map![&env, (1i32.into_val(&env), 2i64.into_val(&env))];
    let map: Val = map.into();
    let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
    map.get(1)
}
```

Use instead:

```rust
pub fn get_map_with_different_values(env: Env, key: i32) -> Result<Option<i32>, Error> {
    let map: Map<Val, Val> = map![
        &env,
        (1i32.into_val(&env), 2i32.into_val(&env)),
        (3i32.into_val(&env), 4i64.into_val(&env)),
    ];
    let map: Val = map.into();
    let map: Map<i32, i32> = map.try_into_val(&env).unwrap();
    map.try_get(key).map_err(Error::from)
}
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-map-get).


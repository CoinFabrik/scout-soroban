# Divide before multiply

### What it does

Checks the existence of a division before a multiplication.

### Why is this bad?

Performing a division operation before multiplication can lead to a loss of precision. It might even result in an unintended zero value.

### Example

```rust
// Example 1 - Vulnerable
let x = 10;
let y = 6;
let z = x / y * 3; // z evaluates to 3

// Example 2 - Vulnerable
let a = 1;
let b = 2;
let c = a / b * 3; // c evaluates to 0
```

Use instead:

```rust
// Example 1 - Remediated
let x = 10;
let y = 6;
let z = x * 3 / y; // z evaluates to 5

// Example 2 - Remediated
let a = 1;
let b = 2;
let c = a * 3 / b; // c evaluates to 1
```

### Implementation

The detector's implementation can be found at [this link](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/divide-before-multiply).

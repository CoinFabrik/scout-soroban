pub const INSUFFICIENTLY_RANDOM_VALUES_LINT_MESSAGE: &str = "Use env.prng() to generate random numbers, and remember that all random numbers are under the control of validators";
pub const DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE: &str =
    "Division before multiplication might result in a loss of precision";
pub const OVERFLOW_CHECK_LINT_MESSAGE: &str = "Use `overflow-checks = true` in Cargo.toml profile";
pub const UNPROTECTED_UPDATE_CURRENT_CONTRACT_MESSAGE: &str =
    "This update_current_contract_wasm is called without access control";
pub const UNSAFE_BLOCK_LINT_MESSAGE: &str = "Avoid using unsafe blocks as it may lead to undefined behavior";
pub const UNSAFE_EXPECT_LINT_MESSAGE: &str = "Unsafe usage of `expect`";
pub const UNSAFE_UNWRAP_MESSAGE: &str = "Unsafe usage of `unwrap`";

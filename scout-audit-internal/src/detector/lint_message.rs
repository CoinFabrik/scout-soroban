pub const DIVIDE_BEFORE_MULTIPLY_LINT_MESSAGE: &str =
    "Division before multiplication might result in a loss of precision";
pub const OVERFLOW_CHECK_LINT_MESSAGE: &str = "Use `overflow-checks = true` in Cargo.toml profile";
pub const SET_CONTRACT_STORAGE_LINT_MESSAGE:&str = "Abitrary users should not have control over keys because it implies writing any value of left mapping, lazy variable, or the main struct of the contract located in position 0 of the storage";
pub const UNSAFE_EXPECT_LINT_MESSAGE: &str = "Unsafe usage of `expect`";
pub const UNSAFE_UNWRAP_LINT_MESSAGE: &str = "Unsafe usage of `unwrap`";

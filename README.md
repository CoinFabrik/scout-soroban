# Scout Soroban: Security Analysis Tool

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

<p align="center">
  <img src="/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

Scout is an extensible open-source tool intended to assist Soroban Stellar smart contract developers and auditors detect common security issues and deviations from best practices.

This tool will help developers write secure and more robust smart contracts.

Our interest in this project comes from our experience in manual auditing and our usage of comparable tools in other blockchains.

Following our work with Scout on Polkadot (see [Scout for ink!](https://github.com/CoinFabrik/scout)), we are planning to develop a CLI and a VS Code plugin that can identify issues in Soroban smart contracts. This tool will include detectors for a prioritized list of vulnerabilities, with the detectors implemented as linters in the Dylint linter.

## Quick Start

For a quick start, make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. Then, install Scout dependencies by running the following command:

```bash
cargo install cargo-dylint dylint-link
```

Afterwards, install Scout with the following command:

```bash
cargo install --path apps/cargo-scout-audit
```

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout-audit
```

## Detectors

| Detector ID                                                                                                              | What it Detects                                                                                                                                                                                           | Test Cases                                                                                                                                                                                                                                               | Severity    |
| ------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| [divide-before-multiply](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/divide-before-multiply)      | Performing a division operation before a multiplication, leading to loss of precision.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1), [2](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-2), [3](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-3) | Medium    |
| [unsafe-unwrap](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-unwrap)      | Inappropriate usage of the unwrap method, causing unexpected program crashes.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1)| Medium    |
| [unsafe-expect](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-expect)       | Improper usage of the expect method, leading to unexpected program crashes.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-expect/unsafe-expect-1)| Medium    |
| [overflow-check](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/overflow-check)       | An arithmetic operation overflows or underflows the available memory allocated to the variable.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/overflow-check/overflow-check-1)| Critical    |
| [insufficiently-random-values](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/insufficiently-random-values)       | Avoid using block attributes for random number generation to prevent manipulation.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1)| Critical    |
| [unprotected-update-current-contract-wasm](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unprotected-update-current-contract-wasm)       | If users are allowed to call `update_current_contract_wasm()`, they can intentionally modify the contract behaviour.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1)| Critical    |
| [unprotected-update-current-contract-wasm](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unprotected-update-current-contract-wasm)       | If users are allowed to call `update_current_contract_wasm()`, they can intentionally modify the contract behaviour.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1)| Critical    |
| [avoid-core-mem-forget](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/avoid-core-mem-forget)                 | The use of `core::mem::forget()` could lead to memory leaks and logic errors.                                                | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/avoid-core-mem-forget/avoid-core-mem-forget-1)                                                                                                                                              | Enhacement  |


## Tests

To validate our tool, we provide a set of code examples located in the [test-cases](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases) folder.

In order to run the integration tests, navigate to `apps/cargo-scout-audit` and run:

```console
cargo test --all --all-features
```

In order to run the tests for a particular test-case, run the same command on that particular test-case folder (e.g: `test-cases/divide-before-multiply/divide-before-multiply-1`)

## Acknowledgements

Scout Soroban is an open source vulnerability analyzer developed by [CoinFabrik's](https://www.coinfabrik.com/) Research and Development team.

We received support through a grant from the [Stellar Community Fund (SCF)](https://communityfund.stellar.org/).

## About CoinFabrik

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, TEAL and Stellar Soroban.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.

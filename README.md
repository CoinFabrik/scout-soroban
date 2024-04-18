# Scout Soroban: Security Analysis Tool

![https://img.shields.io/badge/license-MIT-green](https://img.shields.io/badge/license-MIT-green)

<p align="center">
  <img src="/assets/scout.png" alt="Scout in a Dark Forest" width="300" center  />
</p>

Scout is an extensible open-source tool intended to assist Soroban Stellar smart contract developers and auditors detect common security issues and deviations from best practices.

This tool helps developers write secure and more robust smart contracts.

Our interest in this project comes from our experience in manual auditing and vulnerability detection in other blockchains (see [Scout for ink!](https://github.com/CoinFabrik/scout)).

## Quick Start

For a quick start, make sure that [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed on your computer. Then, install Scout dependencies by running the following command:

```bash
cargo install cargo-dylint dylint-link
```

Afterwards, install Scout with the following command:

```bash
cargo install cargo-scout-audit
```

To run Scout on your project, navigate to its root directory and execute the following command:

```bash
cargo scout-audit
```

For more information on installation and usage, please refer to the [Getting Started](https://coinfabrik.github.io/scout-soroban/docs/intro) section in our documentation below.

# Documentation

- [Getting Started](https://coinfabrik.github.io/scout-soroban/docs/intro)
- [Vulnerabilities](https://coinfabrik.github.io/scout-soroban/docs/vulnerabilities)
- [Detectors](https://coinfabrik.github.io/scout-soroban/docs/detectors)
- [Contribute](https://coinfabrik.github.io/scout-soroban/docs/contribute)
- [Architecture](https://coinfabrik.github.io/scout-soroban/docs/architecture)
- [Blog](https://blog.coinfabrik.com/)

Visit [Scout's website](https://coinfabrik.github.io/scout-soroban/) to view the full documentation.

## Detectors

| Detector ID                                                                                                              | What it Detects                                                                                                                                                                                           | Test Cases                                                                                                                                                                                                                                               | Severity    |
| ------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| [divide-before-multiply](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/divide-before-multiply)      | Performing a division operation before a multiplication, leading to loss of precision.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-1), [2](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-2), [3](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/divide-before-multiply/divide-before-multiply-3) | Medium    |
| [unsafe-unwrap](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-unwrap)      | Inappropriate usage of the unwrap method, causing unexpected program crashes.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-unwrap/unsafe-unwrap-1)| Medium    |
| [unsafe-expect](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unsafe-expect)       | Improper usage of the expect method, leading to unexpected program crashes.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unsafe-expect/unsafe-expect-1)| Medium    |
| [overflow-check](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/overflow-check)       | An arithmetic operation overflows or underflows the available memory allocated to the variable.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/overflow-check/overflow-check-1)| Critical    |
| [insufficiently-random-values](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/insufficiently-random-values)       | Avoid using block attributes for random number generation to prevent manipulation.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/insufficiently-random-values/insufficiently-random-values-1)| Critical    |
| [unprotected-update-current-contract-wasm](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unprotected-update-current-contract-wasm)       | If users are allowed to call `update_current_contract_wasm()`, they can intentionally modify the contract behaviour.    | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unprotected-update-current-contract-wasm/unprotected-update-current-contract-wasm-1)| Critical    |
| [avoid-core-mem-forget](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/avoid-core-mem-forget)                 | The use of `core::mem::forget()` could lead to memory leaks and logic errors.                                                | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/avoid-core-mem-forget/avoid-core-mem-forget-1)  | Enhancement    |
| [set-contract-storage](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/set-contract-storage)                           | Insufficient access control on `env.storage()` method.                                                         | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/set-contract-storage/set-contract-storage-1), [2](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/set-contract-storage/set-contract-storage-2), [3](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/set-contract-storage/set-contract-storage-3)                                                                                                                                                | Critical    |
| [avoid-panic-error](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/avoid-panic-error)                           | Code panics on error instead of using descriptive enum.| [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/avoid-panic-error/avoid-panic-error-1)                                                                                                                                              | Enhancement    |
| [avoid-unsafe-block](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/avoid-unsafe-block)                           | Using unsafe blocks in risks code safety and reliability.| [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/avoid-unsafe-block/avoid-unsafe-block-1)                                                                                                                                              | Critical   |
| [dos-unbounded-operation](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/dos-unbounded-operation)                           | DoS due to unbounded operation. | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-1), [2](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-2), [3](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/dos-unbounded-operation/dos-unbounded-operation-3)                                                                                                                                              | Medium  |
| [soroban-version](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/soroban-version)                           | Using an old version of Soroban can be dangerous, as it may have bugs or security issues. Use the latest version available. | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/soroban-version/soroban-version-1)                 | Enhancement  |
| [unused-return-enum](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/unused-return-enum)                           | Return enum from a function is not completely used. | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unused-return-enum/unused-return-enum-1), [2](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/unused-return-enum/unused-return-enum-2)                   | Minor  |
[iterators-over-indexing](https://github.com/CoinFabrik/scout-soroban/tree/main/detectors/iterators-over-indexing)                           |Iterating with hardcoded indexes is slower than using an iterator. Also, if the index is out of bounds, it will panic. | [1](https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases/iterators-over-indexing-1),                   | Enhancement  |

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

We - [CoinFabrik](https://www.coinfabrik.com/) - are a research and development company specialized in Web3, with a strong background in cybersecurity. Founded in 2014, we have worked on over 180 blockchain-related projects, EVM based and also for Solana, Algorand, Stellar and Polkadot. Beyond development, we offer security audits through a dedicated in-house team of senior cybersecurity professionals, currently working on code in Substrate, Solidity, Clarity, Rust, TEAL and Stellar Soroban.

Our team has an academic background in computer science and mathematics, with work experience focused on cybersecurity and software development, including academic publications, patents turned into products, and conference presentations. Furthermore, we have an ongoing collaboration on knowledge transfer and open-source projects with the University of Buenos Aires.

## License

Scout is licensed and distributed under a MIT license. [Contact us](https://www.coinfabrik.com/) if you're looking for an exception to the terms.

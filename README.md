# Sample Config

[![crates.io page](https://img.shields.io/crates/v/sample-config.svg)](https://crates.io/crates/sample-config)
[![docs.rs page](https://docs.rs/sample-config/badge.svg)](https://docs.rs/sample-config/)
![license: MIT](https://img.shields.io/crates/l/sample-config.svg)

Automatic generation of sample configs with documentation comments. Just derive the `SampleConfig` trait on your config types, create an example object and generate a configuration file using that.

## Usage

Example:

```rust
use sample_config::SampleConfig;

/// Example enum.
#[derive(Debug, Default, SampleConfig)]
enum ExampleEnum {
    /// A.
    #[default]
    VariantA,
}

/// General documentation isn't used.
#[derive(Debug, Default, SampleConfig)]
struct ExampleConfig {
    /// Some optional string.
    string: Option<String>,
    /// Some list of numbers.
    numbers: Vec<usize>,
    /// Enumeration of values.
    value: ExampleEnum,
}

let instance = ExampleConfig::default();
let _yaml_file_string = instance.generate_sample_yaml();
```

Please take a look at the tests to see more complicated examples.

## Lints

This projects uses a bunch of clippy lints for higher code quality and style.

Install [`cargo-lints`](https://github.com/soramitsu/iroha2-cargo_lints) using `cargo install --git https://github.com/FlixCoder/cargo-lints`. The lints are defined in `lints.toml` and can be checked by running `cargo lints clippy --all-targets --workspace`.

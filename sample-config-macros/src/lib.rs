//! Macros for sample configs.
#![allow(clippy::expect_used)] // Allowed in procedural macros.

mod attributes;
mod derive;

use derive::derive_sample_config;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive `SampleConfig` for structs (with doc comments on their fields) and
/// enums.
///
/// Example:
/// ```no_compile
/// use sample_config::SampleConfig;
///
/// #[derive(Debug, SampleConfig)]
/// struct MyConfig {
///     /// My field documentation.
///     my_field: String,
/// }
///
/// let config = MyConfig { my_field: "example".to_owned() };
/// let _yaml_file_string = config.generate_sample_yaml();
/// ```
#[proc_macro_derive(SampleConfig)]
pub fn sample_config(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	derive_sample_config(input).into()
}

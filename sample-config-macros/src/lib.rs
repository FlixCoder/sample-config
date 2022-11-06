//! Macros for sample configs.
#![allow(clippy::expect_used)] // Allowed in procedural macros.

mod attributes;
mod derive;

use derive::derive_sample_config;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derive `SampleConfig` for things that have doc comments on their fields.
#[proc_macro_derive(SampleConfig)]
pub fn sample_config(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	derive_sample_config(input).into()
}

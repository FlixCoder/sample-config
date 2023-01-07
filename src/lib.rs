#![doc = include_str!("../README.md")]

mod implementations;

pub use sample_config_macros::SampleConfig;

/// The type of the sample config output.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputType {
	/// A value is put out.
	Value,
	/// Fields are put out (a struct or arrays).
	Fields,
}

/// Generate sample configs for Rust data constructs automatically using an
/// example instance.
pub trait SampleConfig {
	/// Whether this data construct produces a value (e.g. String) or fields
	/// (e.g. a struct).
	const SAMPLE_OUTPUT_TYPE: OutputType;

	/// Generate a string containing the sample config in Yaml format.
	#[cfg(feature = "yaml")]
	fn generate_sample_yaml(&self) -> String;

	/// Generate a string containing the sample config in JSON format.
	#[cfg(feature = "json")]
	fn generate_sample_json(&self) -> String;
}

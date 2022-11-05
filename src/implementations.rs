//! Implementations of the [SampleConfig] trait for types.

use crate::{OutputType, SampleConfig};

/// Generate implementations for [`SampleConfig`] for string types.
macro_rules! impl_sample_config_for_string {
	([$($string: ty),*$(,)?]) => {
		$(
			impl SampleConfig for $string {
				const SAMPLE_OUTPUT_TYPE: OutputType = OutputType::Value;

				fn generate_sample_yaml(&self) -> String {
					format!("\"{}\"", self)
				}
			}
		)*
	};
}

impl_sample_config_for_string!([String, &str, str]);

/// Generate implementations for [`SampleConfig`] for numeric types.
macro_rules! impl_sample_config_for_number {
	([$($number: ty),*$(,)?]) => {
		$(
			impl SampleConfig for $number {
				const SAMPLE_OUTPUT_TYPE: OutputType = OutputType::Value;

				fn generate_sample_yaml(&self) -> String {
					self.to_string()
				}
			}
		)*
	};
}

impl_sample_config_for_number!([usize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64]);

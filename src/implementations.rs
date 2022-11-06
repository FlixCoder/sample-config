//! Implementations of the [SampleConfig] trait for types.

use std::{
	net::SocketAddr,
	path::{Path, PathBuf},
};

use crate::{OutputType, SampleConfig};

/// Generate implementations for [`SampleConfig`] for types that need `""`
/// around their value and implement `Display`.
macro_rules! impl_sample_config_stringified {
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

impl_sample_config_stringified!([String, str, SocketAddr]);

/// Generate implementations for [`SampleConfig`] for types that just use
/// `to_string` to generate valid output.
macro_rules! impl_sample_config_raw {
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

impl_sample_config_raw!([usize, isize, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64]);

impl<T: SampleConfig> SampleConfig for &T {
	const SAMPLE_OUTPUT_TYPE: OutputType = T::SAMPLE_OUTPUT_TYPE;

	fn generate_sample_yaml(&self) -> String {
		<T as SampleConfig>::generate_sample_yaml(self)
	}
}

impl<T: SampleConfig> SampleConfig for Box<T> {
	const SAMPLE_OUTPUT_TYPE: OutputType = T::SAMPLE_OUTPUT_TYPE;

	fn generate_sample_yaml(&self) -> String {
		<T as SampleConfig>::generate_sample_yaml(self)
	}
}

impl<T: SampleConfig> SampleConfig for Option<T> {
	const SAMPLE_OUTPUT_TYPE: OutputType = T::SAMPLE_OUTPUT_TYPE;

	fn generate_sample_yaml(&self) -> String {
		match self {
			None => "null".to_owned(),
			Some(value) => value.generate_sample_yaml(),
		}
	}
}

impl<T: SampleConfig> SampleConfig for Vec<T> {
	const SAMPLE_OUTPUT_TYPE: OutputType = OutputType::Fields;

	fn generate_sample_yaml(&self) -> String {
		if self.is_empty() {
			return "[]".to_owned();
		}

		let mut sample = String::new();
		for value in self {
			sample.push_str("- ");
			if T::SAMPLE_OUTPUT_TYPE == OutputType::Value {
				sample.push_str(&value.generate_sample_yaml());
			} else {
				sample.push_str("\n  ");
				let sub_sample = value.generate_sample_yaml().replace('\n', "\n  ");
				sample.push_str(sub_sample.trim());
			}
			sample.push('\n');
		}
		sample
	}
}

impl SampleConfig for PathBuf {
	const SAMPLE_OUTPUT_TYPE: OutputType = OutputType::Value;

	fn generate_sample_yaml(&self) -> String {
		format!("\"{}\"", self.display())
	}
}

impl SampleConfig for Path {
	const SAMPLE_OUTPUT_TYPE: OutputType = OutputType::Value;

	fn generate_sample_yaml(&self) -> String {
		format!("\"{}\"", self.display())
	}
}

#![allow(clippy::expect_used)] // Allowed in tests.

use std::{net::SocketAddr, path::PathBuf};

use sample_config::SampleConfig;

/// General documentation isn't used.
#[derive(Debug, SampleConfig)]
struct TestConfig {
	#[doc = "Example field 1."]
	example_sub_struct: ExampleSubConfig,
	/// Example field 2.
	/// Has multiple lines.
	enum_test: ExampleEnum,
	/// A path.
	path: Box<PathBuf>,
	/// Vector of sub-structs.
	vec_of_structs: Vec<ExampleSubSubConfig>,
}

impl Default for TestConfig {
	fn default() -> Self {
		Self {
			example_sub_struct: ExampleSubConfig {
				string: Some("Cats are cute.".to_owned()),
				numbers: vec![1, 2, 3],
				sub_struct: Some(ExampleSubSubConfig {
					some_field: vec!["127.0.0.1:8080"
						.parse()
						.expect("parsing constant socket address")],
				}),
			},
			enum_test: ExampleEnum::VariantA,
			path: Box::new(PathBuf::from("file.txt")),
			vec_of_structs: vec![ExampleSubSubConfig { some_field: vec![] }],
		}
	}
}

/// General documentation isn't used.
#[derive(Debug, SampleConfig)]
struct ExampleSubConfig {
	/// Some optional string.
	string: Option<String>,
	/// Some list of numbers.
	numbers: Vec<usize>,
	/// Sub-sub-struct.
	sub_struct: Option<ExampleSubSubConfig>,
}

/// General documentation isn't used.
#[derive(Debug, SampleConfig)]
struct ExampleSubSubConfig {
	/// Some documentation.
	some_field: Vec<SocketAddr>,
}

/// Example enum.
#[derive(Debug, SampleConfig)]
enum ExampleEnum {
	/// A.
	VariantA,
}

#[test]
fn generated_config() {
	let config = TestConfig::default();
	let generated = config.generate_sample_yaml();
	let expected = include_str!("expected_config.yaml");
	assert_eq!(&generated, expected);
}

#![allow(clippy::expect_used)] // Allowed in tests.

use std::{net::SocketAddr, path::PathBuf};

use sample_config::SampleConfig;
use serde::Deserialize;

/// General documentation isn't used.
#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
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
#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
struct ExampleSubConfig {
	/// Some optional string.
	string: Option<String>,
	/// Some list of numbers.
	numbers: Vec<usize>,
	/// Sub-sub-struct.
	sub_struct: Option<ExampleSubSubConfig>,
}

/// General documentation isn't used.
#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
struct ExampleSubSubConfig {
	/// Some documentation.
	some_field: Vec<SocketAddr>,
}

/// Example enum.
#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
enum ExampleEnum {
	/// A.
	VariantA,
}

#[test]
fn generated_yaml_config() {
	let config = TestConfig::default();
	let generated = config.generate_sample_yaml();
	let expected = include_str!("./expected/config_generation.yaml");
	assert_eq!(&generated, expected);

	let deserialized: TestConfig =
		serde_yaml::from_str(expected).expect("deserialize expected YAML");
	assert_eq!(deserialized, config);
}

#[test]
fn generated_json_config() {
	let config = TestConfig::default();
	let generated = config.generate_sample_json();
	let expected = include_str!("./expected/config_generation.json5");
	assert_eq!(&generated, expected);

	let deserialized: TestConfig = json5::from_str(expected).expect("deserialize expected JSON");
	assert_eq!(deserialized, config);
}

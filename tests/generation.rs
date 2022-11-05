use sample_config::SampleConfig;

/// General documentation.
#[derive(Debug, SampleConfig)]
struct TestConfig {
	/// Cat field.
	cute_cat: CatConfig,
}

impl Default for TestConfig {
	fn default() -> Self {
		Self { cute_cat: CatConfig { head: "fluffy".to_owned(), tails: 1 } }
	}
}

/// General cat documentation.
#[derive(Debug, SampleConfig)]
struct CatConfig {
	#[doc = "Head."]
	head: String,
	/// Number of tails.
	/// Should normally be 1.
	tails: usize,
}

#[test]
fn generated_config() {
	let config = TestConfig::default();
	let generated = config.generate_sample_yaml();
	let expected = include_str!("expected_config.yaml");
	assert_eq!(&generated, expected);
}

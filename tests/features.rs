#![allow(clippy::expect_used)] // Allowed in tests.
#![allow(unused_imports)] // Without features this file is empty.

use sample_config::SampleConfig;
use serde::Deserialize;

#[cfg(feature = "url")]
#[test]
fn url_sample_yaml() {
	use url::Url;

	#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
	struct UrlConfig {
		url: Url,
	}

	let config = UrlConfig { url: "http://example.com/".parse().expect("parsing URL") };
	let generated = config.generate_sample_yaml();
	let expected = include_str!("expected/url_config.yaml");
	assert_eq!(&generated, expected);

	let deserialized: UrlConfig =
		serde_yaml::from_str(expected).expect("deserialize expected YAML");
	assert_eq!(deserialized, config);
}

#[cfg(feature = "url")]
#[test]
fn url_sample_json() {
	use url::Url;

	#[derive(Debug, PartialEq, Eq, SampleConfig, Deserialize)]
	struct UrlConfig {
		url: Url,
	}

	let config = UrlConfig { url: "http://example.com/".parse().expect("parsing URL") };
	let generated = config.generate_sample_json();
	let expected = include_str!("expected/url_config.json5");
	assert_eq!(&generated, expected);

	let deserialized: UrlConfig = json5::from_str(expected).expect("deserialize expected JSON");
	assert_eq!(deserialized, config);
}

#[cfg(feature = "tracing")]
#[test]
fn tracing_sample_yaml() {
	use tracing::{level_filters::LevelFilter, Level};

	#[derive(Debug, SampleConfig)]
	struct TracingConfig {
		/// Level.
		level: Level,
		/// Filter.
		filter: LevelFilter,
	}

	let config = TracingConfig { level: Level::TRACE, filter: LevelFilter::OFF };
	let generated = config.generate_sample_yaml();
	let expected = include_str!("expected/tracing_config.yaml");
	assert_eq!(&generated, expected);
}

#[cfg(feature = "tracing")]
#[test]
fn tracing_sample_json() {
	use tracing::{level_filters::LevelFilter, Level};

	#[derive(Debug, SampleConfig)]
	struct TracingConfig {
		/// Level.
		level: Level,
		/// Filter.
		filter: LevelFilter,
	}

	let config = TracingConfig { level: Level::TRACE, filter: LevelFilter::OFF };
	let generated = config.generate_sample_json();
	let expected = include_str!("expected/tracing_config.json5");
	assert_eq!(&generated, expected);
}

#![allow(clippy::expect_used)] // Allowed in tests.

use sample_config::SampleConfig;

#[cfg(feature = "url")]
#[test]
fn url_sample() {
	use url::Url;

	#[derive(Debug, SampleConfig)]
	struct UrlConfig {
		url: Url,
	}

	let config = UrlConfig { url: "http://example.com/".parse().expect("parsing URL") };
	let generated = config.generate_sample_yaml();
	let expected = include_str!("expected/url_config.yaml");
	assert_eq!(&generated, expected);
}

#[cfg(feature = "tracing")]
#[test]
fn tracing_sample() {
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

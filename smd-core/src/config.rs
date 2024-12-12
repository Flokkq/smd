use crate::error::Result;
use serde::{
	Deserialize,
	Serialize,
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
	/// Configuration values about gfm generation.
	parse: ParseConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParseConfig {
	/// Action to take when a parsing error occurs.
	pub on_parse_error: ParseErrorAction,

	/// Whether to keep temporary files created during conversion.
	pub keep_temp_files: bool,
}

/// Actions to take when a parsing error occurs.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ParseErrorAction {
	/// Abort the entire process on error.
	Abort,
	/// Try to skip the problematic part and continue.
	Skip,
	/// Include the problematic part in the output and continue.
	Serialize,
}

impl Config {
	pub fn parse_from_str(content: &str) -> Result<Config> {
		Ok(config::Config::builder()
			.add_source(config::File::from_str(
				content,
				config::FileFormat::Toml,
			))
			.add_source(config::Environment::with_prefix("SMD").separator("__"))
			.build()?
			.try_deserialize()?)
	}
}

impl Default for ParseErrorAction {
	fn default() -> Self {
		ParseErrorAction::Serialize
	}
}

impl Default for ParseConfig {
	fn default() -> Self {
		Self {
			on_parse_error:  ParseErrorAction::default(),
			keep_temp_files: false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_default_config() {
		let config = Config::default();
		assert_eq!(config.parse.on_parse_error, ParseErrorAction::Serialize);
		assert!(!config.parse.keep_temp_files);
	}

	#[test]
	fn test_parse_from_toml() {
		let toml_content = r#"
            [parse]
            on_parse_error = "abort"
            keep_temp_files = true
        "#;

		let config = Config::parse_from_str(toml_content).unwrap();
		assert_eq!(config.parse.on_parse_error, ParseErrorAction::Abort);
		assert!(config.parse.keep_temp_files);
	}

	// #[test]
	// fn test_environment_override() {
	// 	std::env::set_var("SMD_PARSE__ON_PARSE_ERROR", "skip");
	// 	std::env::set_var("SMD_PARSE__KEEP_TEMP_FILES", "true");
	//
	// 	let toml_content = r#"
	//        [parse]
	//        on_parse_error = "abort"
	//        keep_temp_files = false
	//    "#;
	//
	// 	let config = Config::parse_from_str(toml_content).unwrap();
	//
	// 	assert_eq!(config.parse.on_parse_error, ParseErrorAction::Skip);
	// 	assert!(config.parse.keep_temp_files);
	//
	// 	std::env::remove_var("SMD_PARSE__ON_PARSE_ERROR");
	// 	std::env::remove_var("SMD_PARSE__KEEP_TEMP_FILES");
	// }

	#[test]
	fn test_invalid_toml() {
		let invalid_toml = r#"
            [parse]
            on_parse_error = "invalid_value"
        "#;

		let result = Config::parse_from_str(invalid_toml);
		assert!(result.is_err(), "Expected an error for invalid TOML value.");
	}
}

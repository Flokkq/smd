use crate::error::Error;
use crate::fs;

use crate::{
	error::Result,
	DEFAULT_CONFIG,
};
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
#[derive(Default)]
pub struct ParseConfig {
	/// Action to take when a parsing error occurs.
	pub on_parse_error: ParseErrorAction,

	/// Whether to keep temporary files created during conversion.
	pub keep_temp_files: bool,
}

/// Actions to take when a parsing error occurs.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ParseErrorAction {
	/// Abort the entire process on error.
	Abort,
	/// Try to skip the problematic part and continue.
	Skip,
	/// Include the problematic part in the output and continue.
	#[default]
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

	pub fn load_config() -> Result<Config> {
		let config_path =
			dirs::config_dir().unwrap_or_default().join(DEFAULT_CONFIG);
		if config_path.exists() {
			let content = fs::read_to_string(&config_path)?;
			let config = Self::parse_from_str(&content)?;
			return Ok(config);
		}

		Ok(Self::default())
	}

	pub fn initialize() -> Result<()> {
		let config = Config::default();
		let toml = toml::to_string_pretty(&config)?;

		let path = dirs::config_dir().unwrap_or_default().join(DEFAULT_CONFIG);

		if path.exists() {
			return Err(Error::ConfigAlreadyExistsError(
				path.to_string_lossy().to_string(),
			));
		}

		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		fs::write_to_file(&path, &toml)?;

		Ok(())
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

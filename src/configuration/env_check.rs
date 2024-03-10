use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ConfigurationError {
    MissingConfiguration,
    BrokenConfiguration,
}

impl Display for ConfigurationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::MissingConfiguration => write!(f, "Missing configuration"),
            ConfigurationError::BrokenConfiguration => write!(f, "Broken configuration"),
        }
    }
}

pub fn verify_initial_setup() -> Result<(), ConfigurationError> {
    log::info!("Verifying initial setup...");

    let mut config_folder = dirs::config_dir().unwrap().join("smd");
    if !config_folder.exists() {
        return Err(ConfigurationError::MissingConfiguration);
    }

    Ok(())
}

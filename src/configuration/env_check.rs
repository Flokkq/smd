use std::convert::Infallible;
use std::{borrow::Borrow, path::PathBuf};

use sysinfo::{ProcessorExt, SystemExt};

use crate::api::{APICommunicator, UpdateInfo};

use super::configuration::Settings;

#[derive(thiserror::Error, Debug)]
pub enum ConfigurationError {
    #[error("Missing configuration")]
    MissingConfiguration,
    #[error("Missing configuration")]
    BrokenConfiguration,
}

pub fn verify_initial_setup(
    config_folder: &PathBuf,
) -> Result<(), ConfigurationError> {
    if !config_folder.exists() {
        return Err(ConfigurationError::MissingConfiguration);
    }

    Ok(())
}

pub async fn is_update_available(
    settings: &Settings,
) -> Result<Option<UpdateInfo>, anyhow::Error> {
    let api_communicator = APICommunicator::build(settings.api.clone());

    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let os = system.long_os_version().unwrap_or(String::new());

    let cpu = system
        .processors()
        .iter()
        .next()
        .map(|p| p.brand().to_string())
        .unwrap_or("Uknown".to_string());

    let response = api_communicator
        .get_update(&settings.version, &os, &cpu)
        .await?;

    return Ok(Some(response));
}


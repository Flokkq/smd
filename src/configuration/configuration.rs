use config::Value;
use serde::Deserialize;
use std::{collections::HashMap, path::PathBuf};

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub application: Application,
    pub api: APIConfiguration,
    pub version: String,
}

#[derive(Deserialize, Clone)]
pub struct APIConfiguration {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct Application {
    pub configuration_dir: PathBuf,
}

impl Application {
    fn build() -> Self {
        Self {
            configuration_dir: dirs::config_dir().unwrap().join("smd"),
        }
    }
}

impl Into<Value> for Application {
    fn into(self) -> Value {
        let mut m: HashMap<String, Value> = HashMap::new();
        if let Some(path_str) = self.configuration_dir.to_str() {
            m.insert(
                "configuration_dir".to_string(),
                path_str.to_string().into(),
            );
        }
        Value::from(m)
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    let environment_filename = format!("{}.yaml", environment.as_str());

    let application = Application::build();
    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .set_default("application", application)?
        .build()?;

    let settings: Settings = settings.try_deserialize()?;

    Ok(settings)
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either `local`or `production`.",
                other
            )),
        }
    }
}

use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;
use tracing::*;

/// Use default config if no path is provided
pub fn get_configuration(config: Option<PathBuf>) -> Result<Configuration, config::ConfigError> {
    let mut settings = config::Config::default();

    if let Some(config) = config {
        info!("Using configuration from {:?}", config);
        settings.merge(config::File::with_name(config.to_str().unwrap()))?;
    } else {
        info!("Using dev configuration");
        settings
            .merge(config::File::with_name("configuration/settings"))?
            .merge(config::File::with_name("configuration/dev_settings"))?;
    }

    settings.merge(config::Environment::with_prefix("APP").separator("__"))?;

    settings.try_into()
}

#[derive(serde::Deserialize, Clone)]
pub struct Configuration {
    pub server: ServerSettings,
    pub mqtt: MqttSettings,
    #[serde(default)]
    pub logging_settings: LoggingSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct MqttSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub client_id: String,
    pub username: String,
    pub password: Secret<String>,
}

#[derive(serde::Deserialize, Default, Clone)]
pub struct LoggingSettings {
    #[serde(default)]
    pub log_bunyan: bool,
    #[serde(default)]
    pub log_path: Option<String>,
}

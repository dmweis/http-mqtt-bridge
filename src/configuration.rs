use config::Config;
use secrecy::Secret;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;
use tracing::*;

/// Use default config if no path is provided
pub fn get_configuration(config: &Option<PathBuf>) -> anyhow::Result<Configuration> {
    let settings = if let Some(config) = config {
        info!("Using configuration from {:?}", config);
        Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .add_source(config::File::with_name(
                config
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("Failed to convert path"))?,
            ))
            .build()?
    } else {
        info!("Using dev configuration");
        Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .add_source(config::File::with_name("config/settings"))
            .add_source(config::File::with_name("config/dev_settings"))
            .build()?
    };

    Ok(settings.try_deserialize()?)
}

#[derive(serde::Deserialize, Clone)]
pub struct Configuration {
    pub server: ServerSettings,
    pub bridge: BridgeSettings,
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
pub struct BridgeSettings {
    pub ifttt_key: String,
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

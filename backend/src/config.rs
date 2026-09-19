use serde::Deserialize;
use std::net::IpAddr;
use thiserror::Error;

use authorization::AuthorizationConfig;
use content_access::ContentAccessConfig;
use content_storage::ContentStorageConfig;

pub(crate) mod authorization;
mod content_access;
mod content_storage;

#[derive(Debug, Error)]
/// Represents an error that can occur during configuration validation
pub(crate) enum ConfigError {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("constructing content storage: {0}")]
    ContentStorageConstruction(String),
    #[error("constructing content access configurator: {0}")]
    ContentAccessConstruction(String),
    #[error("constructing authorization: {0}")]
    AuthorizationConstruction(String),
}

// Optionとserdeのデフォルト関数の使い分けは以下の考えに基づいて行う
// Option: 未指定であること自体に意味がある場合
// serdeのデフォルト関数: 未指定を許容するものの内部では必要な値としてデフォルト値を用いる場合

#[derive(Debug, Deserialize)]
/// Represents the configuration for Galerie application.
pub(crate) struct GalerieConfig {
    #[serde(default = "GalerieConfig::default_listen_port")]
    listen_port: u16,
    #[serde(default = "GalerieConfig::default_listen_address")]
    listen_address: String,
    #[serde(default)]
    content_access: ContentAccessConfig,
    #[serde(default)]
    authorization: AuthorizationConfig,
    cors_origin: Option<String>,
    content_storage: ContentStorageConfig,
}

impl GalerieConfig {
    fn default_listen_port() -> u16 {
        3000
    }

    fn default_listen_address() -> String {
        "0.0.0.0".to_string()
    }

    pub(crate) fn content_storage(&self) -> &ContentStorageConfig {
        &self.content_storage
    }

    pub(crate) fn content_access(&self) -> &ContentAccessConfig {
        &self.content_access
    }

    pub(crate) fn authorization(&self) -> &AuthorizationConfig {
        &self.authorization
    }

    pub(crate) fn cors_origin(&self) -> Option<&str> {
        self.cors_origin.as_deref()
    }

    pub(crate) async fn validate(&self) -> Result<(), ConfigError> {
        self.listen_address.parse::<IpAddr>().map_err(|e| {
            ConfigError::InvalidConfig(format!(
                "invalid listen address '{}': {}",
                self.listen_address, e
            ))
        })?;
        self.authorization.validate()?;
        self.content_access.validate()?;
        self.content_storage.validate().await
    }

    pub(crate) fn listen_address(&self) -> String {
        format!("{}:{}", self.listen_address, self.listen_port)
    }
}

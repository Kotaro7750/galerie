use std::sync::Arc;

use rsa::RsaPrivateKey;
use rsa::pkcs8::DecodePrivateKey;
use serde::Deserialize;
use url::Url;

use super::ConfigError;
use crate::infrastructure::content_access::{
    CloudFrontContentAccessConfigurator, NopContentAccessConfigurator,
};
use crate::port::ContentAccessConfigurator;

#[derive(Debug, Default, Deserialize)]
/// Represents the configuration for content access
pub(crate) struct ContentAccessConfig {
    #[serde(default)]
    mode: ContentAccessMode,
    cloud_front: Option<CloudFrontContentAccessConfig>,
}

impl ContentAccessConfig {
    pub(crate) fn construct_content_access_configurator(
        &self,
    ) -> Result<Arc<dyn ContentAccessConfigurator>, ConfigError> {
        match self.mode {
            ContentAccessMode::Nop => Ok(Arc::new(NopContentAccessConfigurator)),
            ContentAccessMode::CloudFront => {
                let config = self.cloud_front.as_ref().expect(
                    "CloudFront content access configuration is required for CloudFront mode",
                );
                let private_key =
                    RsaPrivateKey::from_pkcs8_pem(&config.private_key).map_err(|e| {
                        ConfigError::ContentAccessConstruction(format!(
                            "failed to parse CloudFront PKCS#8 private key: {e}"
                        ))
                    })?;
                Ok(Arc::new(CloudFrontContentAccessConfigurator::new(
                    config.key_pair_id.clone(),
                    private_key,
                    config.resource_url.clone(),
                    config.validity_seconds,
                    config.cookie_domain.clone(),
                    config.cookie_path.clone(),
                )))
            }
        }
    }

    pub(super) fn validate(&self) -> Result<(), ConfigError> {
        match self.mode {
            ContentAccessMode::Nop => Ok(()),
            ContentAccessMode::CloudFront => self
                .cloud_front
                .as_ref()
                .ok_or_else(|| {
                    ConfigError::InvalidConfig(
                        "CloudFront content access configuration is required for CloudFront mode"
                            .to_string(),
                    )
                })?
                .validate(),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
/// Represents the mode of content access
enum ContentAccessMode {
    #[default]
    Nop,
    CloudFront,
}

#[derive(Debug, Deserialize)]
/// Represents the configuration for CloudFront content access
struct CloudFrontContentAccessConfig {
    key_pair_id: String,
    private_key: String,
    resource_url: String,
    validity_seconds: u64,
    cookie_domain: Option<String>,
    #[serde(default = "CloudFrontContentAccessConfig::default_cookie_path")]
    cookie_path: String,
}

impl CloudFrontContentAccessConfig {
    fn default_cookie_path() -> String {
        "/".to_string()
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.key_pair_id.is_empty() {
            return Err(ConfigError::InvalidConfig(
                "CloudFront key pair ID must not be empty".to_string(),
            ));
        }
        if self.private_key.is_empty() {
            return Err(ConfigError::InvalidConfig(
                "CloudFront private key must not be empty".to_string(),
            ));
        }
        if self.validity_seconds == 0 {
            return Err(ConfigError::InvalidConfig(
                "CloudFront content access validity must be greater than zero".to_string(),
            ));
        }
        if self.cookie_path.is_empty() || !self.cookie_path.starts_with('/') {
            return Err(ConfigError::InvalidConfig(
                "content access cookie path must start with '/'".to_string(),
            ));
        }
        Url::parse(&self.resource_url).map_err(|e| {
            ConfigError::InvalidConfig(format!("invalid CloudFront resource URL: {e}"))
        })?;
        Ok(())
    }
}

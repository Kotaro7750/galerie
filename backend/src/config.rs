use std::fs;
use std::net::IpAddr;
use std::sync::Arc;

use aws_config::Region;
use aws_sdk_s3::error::DisplayErrorContext;
use serde::Deserialize;
use thiserror::Error;
use url::Url;

use crate::domain::Error;
use crate::infrastructure::content_storage::filesystem::FileSystemContentStorage;
use crate::infrastructure::content_storage::s3::S3ContentStorage;
use crate::port::ContentStorage;

#[derive(Debug, Error)]
pub(crate) enum ConfigError {
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("constructing content storage: {0}")]
    ContentStorageConstruction(String),
}

#[derive(Debug, Deserialize)]
pub(crate) struct GalerieConfig {
    listen_port: Option<u16>,
    listen_address: Option<String>,
    content_storage: ContentStorageConfig,
}

impl GalerieConfig {
    pub(crate) fn content_storage(&self) -> &ContentStorageConfig {
        &self.content_storage
    }

    pub(crate) async fn validate(&self) -> Result<(), ConfigError> {
        if let Some(listen_address) = self.listen_address.as_ref() {
            listen_address.parse::<IpAddr>().map_err(|e| {
                ConfigError::InvalidConfig(format!(
                    "invalid listen address '{}': {}",
                    listen_address, e
                ))
            })?;
        };
        self.content_storage.validate().await
    }

    pub(crate) fn listen_address_with_default(&self) -> String {
        format!(
            "{}:{}",
            self.listen_address
                .as_ref()
                .unwrap_or(&"0.0.0.0".to_string()),
            self.listen_port.unwrap_or(3000)
        )
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ContentStorageConfig {
    mode: ContentStorageMode,
    file_system: Option<FileSystemContentStorageConfig>,
    s3: Option<S3ContentStorageConfig>,
}

impl ContentStorageConfig {
    pub(crate) async fn construct_content_storage(
        &self,
    ) -> Result<Arc<dyn ContentStorage>, ConfigError> {
        Ok(match self.mode {
            ContentStorageMode::FileSystem => {
                let config = self.file_system.as_ref().expect(
                    "File system content storage configuration is required for FileSystem mode",
                );
                Arc::new(config.construct_content_storage().map_err(|e| {
                    ConfigError::ContentStorageConstruction(format!(
                        "filesystem content storage: {}",
                        e
                    ))
                })?)
            }
            ContentStorageMode::S3 => {
                let config = self
                    .s3
                    .as_ref()
                    .expect("S3 content storage configuration is required for S3 mode");
                Arc::new(config.construct_content_storage().await)
            }
        })
    }

    async fn validate(&self) -> Result<(), ConfigError> {
        match self.mode {
            ContentStorageMode::FileSystem => {
                if let Some(config) = self.file_system.as_ref() {
                    config.validate()
                } else {
                    Err(ConfigError::InvalidConfig(
                        "File system content storage configuration is required for FileSystem mode"
                            .to_string(),
                    ))
                }
            }
            ContentStorageMode::S3 => {
                if let Some(config) = self.s3.as_ref() {
                    config.validate().await
                } else {
                    Err(ConfigError::InvalidConfig(
                        "S3 content storage configuration is required for S3 mode".to_string(),
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
enum ContentStorageMode {
    FileSystem,
    S3,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FileSystemContentStorageConfig {
    content_root_path: String,
    content_url_base: String,
    thumbnail_url_base: String,
}

impl FileSystemContentStorageConfig {
    fn construct_content_storage(&self) -> Result<FileSystemContentStorage, Error> {
        FileSystemContentStorage::new(
            self.content_root_path.clone(),
            self.content_url_base.to_string(),
            self.thumbnail_url_base.to_string(),
        )
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if !fs::exists(self.content_root_path.as_str()).map_err(|e| {
            ConfigError::InvalidConfig(format!("failed to access content root path: {}", e))
        })? {
            return Err(ConfigError::InvalidConfig(format!(
                "content root path does not exist: {}",
                self.content_root_path
            )));
        }

        validate_stream_url(&self.content_url_base, &self.thumbnail_url_base)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct S3ContentStorageConfig {
    region: String,
    bucket_name: String,
    xmp_prefix: String,
    content_prefix: String,
    content_url_base: String,
    thumbnail_url_base: String,
}

impl S3ContentStorageConfig {
    async fn construct_content_storage(&self) -> S3ContentStorage {
        let client = aws_sdk_s3::Client::new(
            &aws_config::load_defaults(aws_config::BehaviorVersion::latest())
                .await
                .into_builder()
                .region(Region::new(self.region.clone()))
                .build(),
        );
        S3ContentStorage::new(
            client,
            self.bucket_name.to_string(),
            self.xmp_prefix.to_string(),
            self.content_prefix.to_string(),
            self.content_url_base.to_string(),
            self.thumbnail_url_base.to_string(),
        )
    }

    async fn validate(&self) -> Result<(), ConfigError> {
        let client = aws_sdk_s3::Client::new(
            &aws_config::load_defaults(aws_config::BehaviorVersion::latest())
                .await
                .into_builder()
                .region(Region::new(self.region.clone()))
                .build(),
        );

        client
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|e| {
                ConfigError::InvalidConfig(format!(
                    "failed to access S3 bucket: {}",
                    DisplayErrorContext(&e)
                ))
            })?;

        validate_stream_url(&self.content_url_base, &self.thumbnail_url_base)
    }
}

fn validate_stream_url(
    content_url_base: &str,
    thumbnail_url_base: &str,
) -> Result<(), ConfigError> {
    Url::parse(content_url_base)
        .map_err(|e| ConfigError::InvalidConfig(format!("invalid content URL base: {}", e)))?;
    Url::parse(thumbnail_url_base)
        .map_err(|e| ConfigError::InvalidConfig(format!("invalid thumbnail URL base: {}", e)))?;
    Ok(())
}

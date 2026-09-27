use std::{fs, sync::Arc};

use aws_config::Region;
use aws_sdk_s3::error::DisplayErrorContext;
use serde::Deserialize;

use super::ConfigError;
use crate::{
    domain::tag_schema::TagSchema,
    infrastructure::tag_schema_storage::{
        DefaultTagSchemaStorage, FileSystemTagSchemaStorage, S3TagSchemaStorage,
    },
    port::TagSchemaStorage,
};

#[derive(Debug, Default, Deserialize)]
pub(crate) struct TagSchemaConfig {
    #[serde(default)]
    mode: TagSchemaStorageMode,
    file_system: Option<FileSystemTagSchemaStorageConfig>,
    s3: Option<S3TagSchemaStorageConfig>,
}

impl TagSchemaConfig {
    pub(crate) async fn construct_schema_storage(
        &self,
    ) -> Result<Arc<dyn TagSchemaStorage>, ConfigError> {
        Ok(match self.mode {
            TagSchemaStorageMode::FileSystem => {
                let config = self.file_system.as_ref().ok_or_else(|| {
                    ConfigError::InvalidConfig(
                        "File system tag schema configuration is required for FileSystem mode"
                            .to_string(),
                    )
                })?;
                Arc::new(config.construct_schema_storage())
            }
            TagSchemaStorageMode::S3 => {
                let config = self.s3.as_ref().ok_or_else(|| {
                    ConfigError::InvalidConfig(
                        "S3 tag schema configuration is required for S3 mode".to_string(),
                    )
                })?;
                Arc::new(config.construct_schema_storage().await)
            }
            TagSchemaStorageMode::None => Arc::new(DefaultTagSchemaStorage::new()),
        })
    }

    pub(super) async fn validate(&self) -> Result<(), ConfigError> {
        match self.mode {
            TagSchemaStorageMode::FileSystem => {
                let Some(config) = self.file_system.as_ref() else {
                    return Err(ConfigError::InvalidConfig(
                        "File system tag schema configuration is required for FileSystem mode"
                            .to_string(),
                    ));
                };
                config.validate()?;
            }
            TagSchemaStorageMode::S3 => {
                let Some(config) = self.s3.as_ref() else {
                    return Err(ConfigError::InvalidConfig(
                        "S3 tag schema configuration is required for S3 mode".to_string(),
                    ));
                };
                config.validate().await?;
            }
            TagSchemaStorageMode::None => {}
        }

        let storage = self.construct_schema_storage().await?;
        let schema: TagSchema = storage.get_tag_schema().await.map_err(|error| {
            ConfigError::TagSchemaConstruction(format!("retrieving tag schema: {error}"))
        })?;
        schema
            .validate_format()
            .map_err(|error| ConfigError::InvalidConfig(format!("invalid tag schema: {error}")))
    }
}

#[derive(Debug, Default, Deserialize)]
/// Represents the storage mode for tag schema.
enum TagSchemaStorageMode {
    #[default]
    None,
    FileSystem,
    S3,
}

#[derive(Debug, Deserialize)]
/// Represents the configuration for file system tag schema storage.
struct FileSystemTagSchemaStorageConfig {
    file_path: String,
}

impl FileSystemTagSchemaStorageConfig {
    fn construct_schema_storage(&self) -> FileSystemTagSchemaStorage {
        FileSystemTagSchemaStorage::new(self.file_path.clone())
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if !fs::metadata(&self.file_path)
            .map_err(|error| {
                ConfigError::InvalidConfig(format!("failed to access tag schema path: {error}"))
            })?
            .is_file()
        {
            return Err(ConfigError::InvalidConfig(format!(
                "tag schema path is not a file: {}",
                self.file_path
            )));
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
/// Represents the configuration for S3 tag schema storage.
struct S3TagSchemaStorageConfig {
    region: String,
    bucket_name: String,
    key: String,
}

impl S3TagSchemaStorageConfig {
    async fn construct_schema_storage(&self) -> S3TagSchemaStorage {
        S3TagSchemaStorage::new(
            self.client().await,
            self.bucket_name.clone(),
            self.key.clone(),
        )
    }

    async fn validate(&self) -> Result<(), ConfigError> {
        self.client()
            .await
            .head_bucket()
            .bucket(&self.bucket_name)
            .send()
            .await
            .map_err(|error| {
                ConfigError::InvalidConfig(format!(
                    "failed to access tag schema S3 bucket: {}",
                    DisplayErrorContext(&error)
                ))
            })?;
        Ok(())
    }

    async fn client(&self) -> aws_sdk_s3::Client {
        aws_sdk_s3::Client::new(
            &aws_config::load_defaults(aws_config::BehaviorVersion::latest())
                .await
                .into_builder()
                .region(Region::new(self.region.clone()))
                .build(),
        )
    }
}

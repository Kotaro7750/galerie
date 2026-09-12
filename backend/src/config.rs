use std::sync::Arc;

use aws_config::Region;
use serde::Deserialize;

use crate::domain::Error;
use crate::infrastructure::content_storage::filesystem::FileSystemContentStorage;
use crate::infrastructure::content_storage::s3::S3ContentStorage;
use crate::port::ContentStorage;

#[derive(Debug, Deserialize)]
pub(crate) struct GalerieConfig {
    content_storage: ContentStorageConfig,
}

impl GalerieConfig {
    pub(crate) fn content_storage(&self) -> &ContentStorageConfig {
        &self.content_storage
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ContentStorageConfig {
    mode: ContentStorageMode,
    file_system: Option<FileSystemContentStorageConfig>,
    s3: Option<S3ContentStorageConfig>,
}

impl ContentStorageConfig {
    pub(crate) async fn construct_content_storage(&self) -> Result<Arc<dyn ContentStorage>, Error> {
        Ok(match self.mode {
            ContentStorageMode::FileSystem => {
                let config = self.file_system.as_ref().expect(
                    "File system content storage configuration is required for FileSystem mode",
                );
                Arc::new(config.construct_content_storage().await.ok_or_else(|| {
                    Error::Internal("Failed to construct file system content storage".to_string())
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
    async fn construct_content_storage(&self) -> Option<FileSystemContentStorage> {
        FileSystemContentStorage::new(
            self.content_root_path.clone(),
            self.content_url_base.to_string(),
            self.thumbnail_url_base.to_string(),
        )
        .ok()
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
}

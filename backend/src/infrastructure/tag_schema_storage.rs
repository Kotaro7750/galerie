use std::{fs, path::PathBuf};

use async_trait::async_trait;
use aws_sdk_s3::Client;

use crate::{
    domain::{Error, tag_schema::TagSchema},
    port::TagSchemaStorage,
};

#[derive(Debug)]
/// A tag schema storage that reads the tag schema from a file.
pub(crate) struct FileSystemTagSchemaStorage {
    file_path: PathBuf,
}

impl FileSystemTagSchemaStorage {
    pub(crate) fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

#[async_trait]
impl TagSchemaStorage for FileSystemTagSchemaStorage {
    async fn get_tag_schema(&self) -> Result<TagSchema, Error> {
        let source = fs::read_to_string(&self.file_path).map_err(|error| {
            Error::Internal(format!(
                "failed to read tag schema '{}': {error}",
                self.file_path.display()
            ))
        })?;
        serde_yaml::from_str(&source).map_err(|error| {
            Error::Internal(format!(
                "failed to deserialize tag schema '{}': {error}",
                self.file_path.display()
            ))
        })
    }
}

#[derive(Debug)]
/// A tag schema storage that reads the tag schema from an S3 bucket.
pub(crate) struct S3TagSchemaStorage {
    client: Client,
    bucket_name: String,
    key: String,
}

impl S3TagSchemaStorage {
    pub(crate) fn new(client: Client, bucket_name: String, key: String) -> Self {
        Self {
            client,
            bucket_name,
            key,
        }
    }
}

#[async_trait]
impl TagSchemaStorage for S3TagSchemaStorage {
    async fn get_tag_schema(&self) -> Result<TagSchema, Error> {
        let bytes = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(&self.key)
            .send()
            .await
            .map_err(|error| Error::Internal(format!("failed to get tag schema: {error}")))?
            .body
            .collect()
            .await
            .map_err(|error| Error::Internal(format!("failed to read tag schema: {error}")))?
            .into_bytes();
        let source = std::str::from_utf8(&bytes)
            .map_err(|error| Error::Internal(format!("tag schema is not UTF-8: {error}")))?;

        serde_yaml::from_str(source)
            .map_err(|error| Error::Internal(format!("failed to deserialize tag schema: {error}")))
    }
}

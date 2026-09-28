use async_trait::async_trait;
use std::num::NonZeroU64;

use crate::domain::{
    Content, ContentId, Error, MediaType, content_access::ContentAccessConfiguration,
    search_condition::SearchCondition, tag_schema::TagSchema,
};
use url::Url;

pub(crate) trait ContentAccessConfigurator: Send + Sync {
    fn configure(&self) -> Result<ContentAccessConfiguration, Error>;
    fn clear(&self) -> Result<ContentAccessConfiguration, Error>;
}

#[async_trait]
pub(crate) trait ContentStorage: Send + Sync {
    /// Create only the content file. Clean up an incomplete write on failure.
    async fn create_content_file(
        &self,
        id: ContentId,
        media_type: MediaType,
        content_bytes: &[u8],
    ) -> Result<(Url, Url), Error>;

    /// Create only the XMP sidecar. Clean up an incomplete write on failure.
    async fn create_xmp_sidecar(&self, id: ContentId, xmp: &str) -> Result<(), Error>;

    /// Remove the content file created for this id when sidecar creation fails.
    async fn delete_content_file(&self, id: ContentId, media_type: MediaType) -> Result<(), Error>;

    /// Scan content storage and return a sorted list of content in ascending order by content id.
    /// This method returns at most `limit` number of content, and a cursor to continue scanning from the last content id.
    async fn scan_contents(
        &self,
        tag_schema: &TagSchema,
        limit: NonZeroU64,
        cursor: Option<ContentId>,
    ) -> Result<(Vec<Content>, Option<ContentId>), Error>;
}

#[async_trait]
pub(crate) trait TagSchemaStorage: Send + Sync {
    async fn get_tag_schema(&self) -> Result<TagSchema, Error>;
}

#[async_trait]
pub(crate) trait MetadataIndex: Send + Sync {
    async fn add_contents(&self, contents: &[Content]) -> Result<(), Error>;
    async fn get_content(&self, id: ContentId) -> Result<Content, Error>;
    async fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
        search_condition: Option<SearchCondition>,
    ) -> Result<(Vec<Content>, Option<String>), Error>;
}

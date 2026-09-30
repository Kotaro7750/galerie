use std::sync::Arc;

use crate::domain::search_condition::SearchCondition;
use crate::domain::tag::xmp::serialize::{create_xmp_with_tags, modify_xmp_with_tags};
use crate::domain::{
    Content, ContentId, Error, MediaType, content_access::ContentAccessConfiguration, tag::TagSet,
};
use crate::port::{ContentAccessConfigurator, ContentStorage, MetadataIndex};

#[derive(Clone)]
pub(crate) struct CreateContentUseCase {
    content_storage: Arc<dyn ContentStorage>,
    metadata_index: Arc<dyn MetadataIndex>,
}

impl CreateContentUseCase {
    pub(crate) fn new(
        content_storage: Arc<dyn ContentStorage>,
        metadata_index: Arc<dyn MetadataIndex>,
    ) -> Self {
        Self {
            content_storage,
            metadata_index,
        }
    }

    pub(crate) async fn execute(
        &self,
        media_type: MediaType,
        content_bytes: &[u8],
        tags: TagSet,
    ) -> Result<Content, Error> {
        let id = uuid::Uuid::new_v4()
            .try_into()
            .map_err(|e| Error::Internal(format!("Failed to generate content ID: {e}")))?;
        let xmp = create_xmp_with_tags(&tags)
            .map_err(|e| Error::Internal(format!("Failed to generate XMP metadata: {e}")))?;

        let (content_url, thumbnail_url) = self
            .content_storage
            .create_content_file(id, media_type, content_bytes)
            .await?;
        if let Err(error) = self.content_storage.create_xmp_sidecar(id, &xmp).await {
            if let Err(cleanup_error) = self
                .content_storage
                .delete_content_file(id, media_type)
                .await
            {
                tracing::warn!(%cleanup_error, content_id = %id.as_ref(), "Failed to remove content file after XMP creation failed");
            }
            return Err(error);
        }

        let content = Content::new(
            id,
            media_type,
            content_url,
            thumbnail_url,
            tags,
            Default::default(),
        );

        if let Err(error) = self.metadata_index.add_contents(&[content.clone()]).await {
            tracing::error!(%error, content_id = %id.as_ref(), "Content stored but index update failed");
        }

        Ok(content)
    }
}

#[derive(Clone)]
pub(crate) struct UpdateContentUseCase {
    content_storage: Arc<dyn ContentStorage>,
    metadata_index: Arc<dyn MetadataIndex>,
}

impl UpdateContentUseCase {
    pub(crate) fn new(
        content_storage: Arc<dyn ContentStorage>,
        metadata_index: Arc<dyn MetadataIndex>,
    ) -> Self {
        Self {
            content_storage,
            metadata_index,
        }
    }

    pub(crate) async fn execute(&self, id: ContentId, tags: TagSet) -> Result<Content, Error> {
        let old = self.metadata_index.get_content(id).await?;
        let xmp = self.content_storage.get_xmp_sidecar(id).await?;

        let updated_xmp = modify_xmp_with_tags(&xmp, &tags)
            .map_err(|e| Error::Internal(format!("Failed to update XMP metadata: {e}")))?;

        self.content_storage
            .replace_xmp_sidecar(id, &updated_xmp)
            .await?;

        let content = Content::new(
            id,
            old.media_type(),
            old.content_url().clone(),
            old.thumbnail_url().clone(),
            tags,
            Default::default(),
        );

        if let Err(error) = self.metadata_index.add_contents(&[content.clone()]).await {
            tracing::error!(%error, content_id = %id.as_ref(), "XMP updated but index update failed");
        }
        Ok(content)
    }
}

#[derive(Clone)]
pub(crate) struct DeleteContentUseCase {
    content_storage: Arc<dyn ContentStorage>,
    metadata_index: Arc<dyn MetadataIndex>,
}

impl DeleteContentUseCase {
    pub(crate) fn new(
        content_storage: Arc<dyn ContentStorage>,
        metadata_index: Arc<dyn MetadataIndex>,
    ) -> Self {
        Self {
            content_storage,
            metadata_index,
        }
    }

    pub(crate) async fn execute(&self, id: ContentId) -> Result<(), Error> {
        let content = self.metadata_index.get_content(id).await?;

        self.content_storage
            .delete_content_file(id, content.media_type())
            .await?;

        if let Err(error) = self.content_storage.delete_xmp_sidecar(id).await {
            tracing::warn!(%error, content_id = %id.as_ref(), "Content removed but XMP removal failed");
        }
        if let Err(error) = self.metadata_index.remove_content(id).await {
            tracing::error!(%error, content_id = %id.as_ref(), "Content removed but index removal failed");
        }
        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct ConfigureContentAccessUseCase {
    configurator: Arc<dyn ContentAccessConfigurator>,
}

impl ConfigureContentAccessUseCase {
    pub(crate) fn new(configurator: Arc<dyn ContentAccessConfigurator>) -> Self {
        Self { configurator }
    }

    pub(crate) fn execute(&self) -> Result<ContentAccessConfiguration, Error> {
        self.configurator.configure()
    }
}

#[derive(Clone)]
pub(crate) struct ClearContentAccessUseCase {
    configurator: Arc<dyn ContentAccessConfigurator>,
}

impl ClearContentAccessUseCase {
    pub(crate) fn new(configurator: Arc<dyn ContentAccessConfigurator>) -> Self {
        Self { configurator }
    }

    pub(crate) fn execute(&self) -> Result<ContentAccessConfiguration, Error> {
        self.configurator.clear()
    }
}

#[derive(Clone)]
pub(crate) struct GetContentUseCase {
    metadata_index: Arc<dyn MetadataIndex>,
}

impl GetContentUseCase {
    pub(crate) fn new(metadata_index: Arc<dyn MetadataIndex>) -> Self {
        Self { metadata_index }
    }

    pub(crate) async fn execute(&self, id: ContentId) -> Result<Content, Error> {
        self.metadata_index.as_ref().get_content(id).await
    }
}

#[derive(Clone)]
pub(crate) struct ListContentsUseCase {
    metadata_index: Arc<dyn MetadataIndex>,
}

impl ListContentsUseCase {
    pub(crate) fn new(metadata_index: Arc<dyn MetadataIndex>) -> Self {
        Self { metadata_index }
    }

    pub(crate) async fn execute(
        &self,
        limit: u64,
        cursor: Option<String>,
        search_condition: Option<SearchCondition>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        self.metadata_index
            .as_ref()
            .list_contents(limit, cursor, search_condition)
            .await
    }
}

#[cfg(test)]
mod create_tests {
    use super::*;
    use crate::domain::{MediaType, tag_schema::TagSchema};
    use async_trait::async_trait;
    use std::num::NonZeroU64;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Storage {
        fail_content: bool,
        fail_xmp: bool,
        fail_delete: bool,
        content_calls: Arc<AtomicUsize>,
        xmp_calls: Arc<AtomicUsize>,
        delete_calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl ContentStorage for Storage {
        async fn create_content_file(
            &self,
            _: ContentId,
            _: MediaType,
            _: &[u8],
        ) -> Result<(url::Url, url::Url), Error> {
            self.content_calls.fetch_add(1, Ordering::SeqCst);
            if self.fail_content {
                return Err(Error::Internal("storage failed".to_string()));
            }
            Ok((
                "https://example.com/content.avif".parse().unwrap(),
                "https://example.com/thumbnail.avif".parse().unwrap(),
            ))
        }

        async fn create_xmp_sidecar(&self, _: ContentId, _: &str) -> Result<(), Error> {
            self.xmp_calls.fetch_add(1, Ordering::SeqCst);
            if self.fail_xmp {
                Err(Error::Internal("xmp failed".to_string()))
            } else {
                Ok(())
            }
        }

        async fn delete_content_file(&self, _: ContentId, _: MediaType) -> Result<(), Error> {
            self.delete_calls.fetch_add(1, Ordering::SeqCst);
            if self.fail_delete {
                Err(Error::Internal("delete failed".to_string()))
            } else {
                Ok(())
            }
        }
        async fn delete_xmp_sidecar(&self, _: ContentId) -> Result<(), Error> {
            unreachable!()
        }
        async fn get_xmp_sidecar(&self, _: ContentId) -> Result<String, Error> {
            unreachable!()
        }
        async fn replace_xmp_sidecar(&self, _: ContentId, _: &str) -> Result<(), Error> {
            unreachable!()
        }

        async fn scan_contents(
            &self,
            _: &TagSchema,
            _: NonZeroU64,
            _: Option<ContentId>,
        ) -> Result<(Vec<Content>, Option<ContentId>), Error> {
            unreachable!()
        }
    }

    struct FailingIndex {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MetadataIndex for FailingIndex {
        async fn add_contents(&self, contents: &[Content]) -> Result<(), Error> {
            assert_eq!(contents.len(), 1);
            self.calls.fetch_add(1, Ordering::SeqCst);
            Err(Error::Internal("index failed".to_string()))
        }
        async fn get_content(&self, _: ContentId) -> Result<Content, Error> {
            unreachable!()
        }
        async fn remove_content(&self, _: ContentId) -> Result<(), Error> {
            unreachable!()
        }
        async fn list_contents(
            &self,
            _: u64,
            _: Option<String>,
            _: Option<SearchCondition>,
        ) -> Result<(Vec<Content>, Option<String>), Error> {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn create_content_use_case_execute_skips_xmp_and_compensation_when_content_creation_fails()
     {
        let xmp_calls = Arc::new(AtomicUsize::new(0));
        let delete_calls = Arc::new(AtomicUsize::new(0));
        let index_calls = Arc::new(AtomicUsize::new(0));
        let usecase = CreateContentUseCase::new(
            Arc::new(Storage {
                fail_content: true,
                fail_xmp: false,
                fail_delete: false,
                content_calls: Arc::new(AtomicUsize::new(0)),
                xmp_calls: xmp_calls.clone(),
                delete_calls: delete_calls.clone(),
            }),
            Arc::new(FailingIndex {
                calls: index_calls.clone(),
            }),
        );
        assert!(
            usecase
                .execute(MediaType::Avif, b"content", TagSet::new(&[]).unwrap())
                .await
                .is_err()
        );
        assert_eq!(xmp_calls.load(Ordering::SeqCst), 0);
        assert_eq!(delete_calls.load(Ordering::SeqCst), 0);
        assert_eq!(index_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn create_content_use_case_execute_deletes_content_and_skips_index_when_xmp_creation_fails()
     {
        let content_calls = Arc::new(AtomicUsize::new(0));
        let xmp_calls = Arc::new(AtomicUsize::new(0));
        let delete_calls = Arc::new(AtomicUsize::new(0));
        let index_calls = Arc::new(AtomicUsize::new(0));
        let index = Arc::new(FailingIndex {
            calls: index_calls.clone(),
        });
        let usecase = CreateContentUseCase::new(
            Arc::new(Storage {
                fail_content: false,
                fail_xmp: true,
                fail_delete: true,
                content_calls: content_calls.clone(),
                xmp_calls: xmp_calls.clone(),
                delete_calls: delete_calls.clone(),
            }),
            index.clone(),
        );
        assert!(
            usecase
                .execute(MediaType::Avif, b"content", TagSet::new(&[]).unwrap())
                .await
                .is_err()
        );
        assert_eq!(content_calls.load(Ordering::SeqCst), 1);
        assert_eq!(xmp_calls.load(Ordering::SeqCst), 1);
        assert_eq!(delete_calls.load(Ordering::SeqCst), 1);
        assert_eq!(index_calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn create_content_use_case_execute_succeeds_when_index_update_fails() {
        let index_calls = Arc::new(AtomicUsize::new(0));
        let index = Arc::new(FailingIndex {
            calls: index_calls.clone(),
        });
        let succeeded = CreateContentUseCase::new(
            Arc::new(Storage {
                fail_content: false,
                fail_xmp: false,
                fail_delete: false,
                content_calls: Arc::new(AtomicUsize::new(0)),
                xmp_calls: Arc::new(AtomicUsize::new(0)),
                delete_calls: Arc::new(AtomicUsize::new(0)),
            }),
            index,
        );
        assert!(
            succeeded
                .execute(MediaType::Avif, b"content", TagSet::new(&[]).unwrap())
                .await
                .is_ok()
        );
        assert_eq!(index_calls.load(Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
mod delete_tests {
    use super::*;
    use crate::domain::tag_schema::TagSchema;
    use async_trait::async_trait;
    use std::num::NonZeroU64;
    use std::sync::Mutex;

    struct Storage {
        events: Arc<Mutex<Vec<&'static str>>>,
        fail_content: bool,
        fail_xmp: bool,
    }
    #[async_trait]
    impl ContentStorage for Storage {
        async fn create_content_file(
            &self,
            _: ContentId,
            _: MediaType,
            _: &[u8],
        ) -> Result<(url::Url, url::Url), Error> {
            unreachable!()
        }
        async fn create_xmp_sidecar(&self, _: ContentId, _: &str) -> Result<(), Error> {
            unreachable!()
        }
        async fn delete_content_file(&self, _: ContentId, _: MediaType) -> Result<(), Error> {
            self.events.lock().unwrap().push("content");
            if self.fail_content {
                Err(Error::Internal("content".into()))
            } else {
                Ok(())
            }
        }
        async fn delete_xmp_sidecar(&self, _: ContentId) -> Result<(), Error> {
            self.events.lock().unwrap().push("xmp");
            if self.fail_xmp {
                Err(Error::Internal("xmp".into()))
            } else {
                Ok(())
            }
        }
        async fn get_xmp_sidecar(&self, _: ContentId) -> Result<String, Error> {
            unreachable!()
        }
        async fn replace_xmp_sidecar(&self, _: ContentId, _: &str) -> Result<(), Error> {
            unreachable!()
        }
        async fn scan_contents(
            &self,
            _: &TagSchema,
            _: NonZeroU64,
            _: Option<ContentId>,
        ) -> Result<(Vec<Content>, Option<ContentId>), Error> {
            unreachable!()
        }
    }
    struct Index {
        content: Content,
        events: Arc<Mutex<Vec<&'static str>>>,
    }
    #[async_trait]
    impl MetadataIndex for Index {
        async fn add_contents(&self, _: &[Content]) -> Result<(), Error> {
            unreachable!()
        }
        async fn get_content(&self, _: ContentId) -> Result<Content, Error> {
            Ok(self.content.clone())
        }
        async fn remove_content(&self, _: ContentId) -> Result<(), Error> {
            self.events.lock().unwrap().push("index");
            Ok(())
        }
        async fn list_contents(
            &self,
            _: u64,
            _: Option<String>,
            _: Option<SearchCondition>,
        ) -> Result<(Vec<Content>, Option<String>), Error> {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn delete_stops_on_content_failure_and_continues_after_xmp_failure() {
        let id = uuid::Uuid::new_v4().try_into().unwrap();
        let content = Content::new(
            id,
            MediaType::Avif,
            "https://example.com/a.avif".parse().unwrap(),
            "https://example.com/a.avif".parse().unwrap(),
            TagSet::new(&[]).unwrap(),
            Default::default(),
        );
        for (fail_content, expected, success) in [
            (true, vec!["content"], false),
            (false, vec!["content", "xmp", "index"], true),
        ] {
            let events = Arc::new(Mutex::new(Vec::new()));
            let usecase = DeleteContentUseCase::new(
                Arc::new(Storage {
                    events: events.clone(),
                    fail_content,
                    fail_xmp: true,
                }),
                Arc::new(Index {
                    content: content.clone(),
                    events: events.clone(),
                }),
            );
            assert_eq!(usecase.execute(id).await.is_ok(), success);
            assert_eq!(*events.lock().unwrap(), expected);
        }
    }
}

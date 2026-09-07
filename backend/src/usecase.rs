use std::sync::Arc;

use crate::domain::{Content, ContentId, Error};
use crate::port::MetadataIndex;

#[derive(Clone)]
pub(crate) struct GetContentUseCase {
    metadata_index: Arc<dyn MetadataIndex>,
}

impl GetContentUseCase {
    pub(crate) fn new(metadata_index: Arc<dyn MetadataIndex>) -> Self {
        Self { metadata_index }
    }

    pub(crate) fn execute(&self, id: ContentId) -> Result<Content, Error> {
        self.metadata_index.as_ref().get_content(id)
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

    pub(crate) fn execute(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        self.metadata_index.as_ref().list_contents(limit, cursor)
    }
}

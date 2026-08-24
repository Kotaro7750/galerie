use std::sync::Arc;

use crate::domain::{Content, ContentId, Error};
use crate::port::ContentStorage;

#[derive(Clone)]
pub(crate) struct GetContentUseCase {
    content_storage: Arc<dyn ContentStorage>,
}

impl GetContentUseCase {
    pub(crate) fn new(content_storage: Arc<dyn ContentStorage>) -> Self {
        Self { content_storage }
    }

    pub(crate) fn execute(&self, id: ContentId) -> Result<Content, Error> {
        self.content_storage.as_ref().get_content(id)
    }
}

#[derive(Clone)]
pub(crate) struct ListContentsUseCase {
    content_storage: Arc<dyn ContentStorage>,
}

impl ListContentsUseCase {
    pub(crate) fn new(content_storage: Arc<dyn ContentStorage>) -> Self {
        Self { content_storage }
    }

    pub(crate) fn execute(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        self.content_storage.as_ref().list_contents(limit, cursor)
    }
}

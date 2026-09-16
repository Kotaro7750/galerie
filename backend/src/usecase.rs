use std::sync::Arc;

use crate::domain::search_condition::SearchCondition;
use crate::domain::{Content, ContentId, Error, content_access::ContentAccessConfiguration};
use crate::port::{ContentAccessConfigurator, MetadataIndex};

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
        search_condition: Option<SearchCondition>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        self.metadata_index
            .as_ref()
            .list_contents(limit, cursor, search_condition)
    }
}

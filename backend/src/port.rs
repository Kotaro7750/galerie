use std::num::NonZeroU64;

use crate::domain::{Content, ContentId, Error, search_condition::SearchCondition};

pub(crate) trait ContentStorage: Send + Sync {
    /// Scan content storage and return a sorted list of content in ascending order by content id.
    /// This method returns at most `limit` number of content, and a cursor to continue scanning from the last content id.
    fn scan_contents(
        &self,
        limit: NonZeroU64,
        cursor: Option<ContentId>,
    ) -> Result<(Vec<Content>, Option<ContentId>), Error>;
}

pub(crate) trait MetadataIndex: Send + Sync {
    fn add_contents(&mut self, contents: &[Content]) -> Result<(), Error>;
    fn get_content(&self, id: ContentId) -> Result<Content, Error>;
    fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
        search_condition: Option<SearchCondition>,
    ) -> Result<(Vec<Content>, Option<String>), Error>;
}

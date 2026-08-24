use crate::domain::{Content, ContentId, Error};

pub(crate) trait ContentStorage: Send + Sync {
    fn get_content(&self, id: ContentId) -> Result<Content, Error>;
    fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error>;
}

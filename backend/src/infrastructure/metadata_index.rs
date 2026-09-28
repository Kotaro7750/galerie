use std::collections::BTreeMap;
use std::str::FromStr;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::domain::search_condition::{ContentConditionMatcher, SearchCondition};
use crate::domain::{Content, ContentId, Error};
use crate::port::MetadataIndex;

#[derive(Debug)]
pub(crate) struct InMemoryMetadataIndex {
    sorted_contents: RwLock<BTreeMap<ContentId, Content>>,
}

impl InMemoryMetadataIndex {
    pub(crate) fn new() -> Self {
        Self {
            sorted_contents: RwLock::new(BTreeMap::new()),
        }
    }

    fn id_to_cursor(&self, id: ContentId) -> String {
        id.as_ref().as_hyphenated().to_string()
    }

    fn cursor_to_id(&self, cursor: String) -> Option<ContentId> {
        ContentId::from_str(&cursor).ok()
    }
}

#[async_trait]
impl MetadataIndex for InMemoryMetadataIndex {
    async fn add_contents(&self, contents: &[Content]) -> Result<(), Error> {
        let mut new_contents = contents
            .iter()
            .map(|content| (content.id(), content.clone()))
            .collect();

        self.sorted_contents.write().await.append(&mut new_contents);
        Ok(())
    }

    async fn get_content(&self, id: ContentId) -> Result<Content, Error> {
        self.sorted_contents
            .read()
            .await
            .get(&id)
            .ok_or(Error::ContentNotFound)
            .cloned()
    }

    async fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
        search_condition: Option<SearchCondition>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        let sorted_contents = self.sorted_contents.read().await;
        let first_element_key = match cursor {
            Some(cursor) => {
                let cursor_content_id = self.cursor_to_id(cursor).ok_or(Error::InvalidCursor)?;

                if let Some(content) = sorted_contents.get(&cursor_content_id) {
                    if let Some(search_condition) = &search_condition
                        && !search_condition.is_match(content)
                    {
                        return Err(Error::InvalidCursor);
                    } else {
                        cursor_content_id
                    }
                } else {
                    return Err(Error::InvalidCursor);
                }
            }
            None => {
                if let Some((first_content_id, _)) = sorted_contents.first_key_value() {
                    *first_content_id
                } else {
                    return Ok((Vec::new(), None));
                }
            }
        };

        let contents = sorted_contents
            .range(first_element_key..)
            .filter(|(_, content)| {
                if let Some(search_condition) = &search_condition {
                    search_condition.is_match(content)
                } else {
                    true
                }
            })
            .take((limit + 1) as usize) // +1 for next cursor
            .map(|(_, content)| content.clone())
            .collect::<Vec<Content>>();

        if contents.len() > limit as usize {
            let next_cursor = self.id_to_cursor(contents[limit as usize].id());
            Ok((contents[..limit as usize].to_vec(), Some(next_cursor)))
        } else {
            Ok((contents, None))
        }
    }
}

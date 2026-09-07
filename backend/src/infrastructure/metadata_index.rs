use std::collections::BTreeMap;
use std::str::FromStr;

use crate::domain::{Content, ContentId, Error};
use crate::port::MetadataIndex;

#[derive(Debug, Clone)]
pub(crate) struct InMemoryMetadataIndex {
    sorted_contents: BTreeMap<ContentId, Content>,
}

impl InMemoryMetadataIndex {
    pub(crate) fn new() -> Self {
        Self {
            sorted_contents: BTreeMap::new(),
        }
    }

    fn id_to_cursor(&self, id: ContentId) -> String {
        id.as_ref().as_hyphenated().to_string()
    }

    fn cursor_to_id(&self, cursor: String) -> Option<ContentId> {
        ContentId::from_str(&cursor).ok()
    }
}

impl MetadataIndex for InMemoryMetadataIndex {
    fn add_contents(&mut self, contents: &[Content]) -> Result<(), Error> {
        let mut new_contents = contents
            .iter()
            .map(|content| (content.id(), content.clone()))
            .collect();

        self.sorted_contents.append(&mut new_contents);
        Ok(())
    }

    fn get_content(&self, id: ContentId) -> Result<Content, Error> {
        self.sorted_contents
            .get(&id)
            .ok_or(Error::ContentNotFound)
            .cloned()
    }

    fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        let first_element_key = match cursor {
            Some(cursor) => {
                let cursor_content_id = self.cursor_to_id(cursor).ok_or(Error::InvalidCursor)?;

                if self.sorted_contents.contains_key(&cursor_content_id) {
                    cursor_content_id
                } else {
                    return Err(Error::InvalidCursor);
                }
            }
            None => {
                if let Some((first_content_id, _)) = self.sorted_contents.first_key_value() {
                    *first_content_id
                } else {
                    return Ok((Vec::new(), None));
                }
            }
        };

        let contents = self
            .sorted_contents
            .range(first_element_key..)
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

use std::fs::{self};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::domain::{Content, ContentId, Error, MediaType};
use crate::port::ContentStorage;

#[derive(Debug)]
pub(crate) struct FileSystemContentStorage {
    contents_directory: PathBuf,
    content_url_base: String,
    thumbnail_url_base: String,
}

impl FileSystemContentStorage {
    pub(crate) fn new<P>(
        content_directory_path: P,
        content_url_base: P,
        thumbnail_url_base: P,
    ) -> Result<Self, Error>
    where
        P: AsRef<str>,
    {
        let contents_directory = PathBuf::from(content_directory_path.as_ref());

        if fs::metadata(&contents_directory)
            .map_err(|e| {
                Error::Internal(format!(
                    "Failed to access contents directory: {}",
                    e.to_string()
                ))
            })?
            .is_dir()
        {
            Ok(Self {
                contents_directory,
                content_url_base: content_url_base.as_ref().to_string(),
                thumbnail_url_base: thumbnail_url_base.as_ref().to_string(),
            })
        } else {
            Err(Error::Internal(format!(
                "Contents directory is not a directory: {}",
                contents_directory.display()
            )))
        }
    }

    fn xmp_file_path(&self, id: ContentId) -> PathBuf {
        todo!();
    }

    fn content_file_path(&self, id: ContentId) -> PathBuf {
        self.contents_directory
            .clone()
            .join(format!("{}.avif", id.as_ref()))
    }

    fn generate_content(&self, content_file_path: &Path) -> Option<Content> {
        if !fs::metadata(content_file_path).ok()?.file_type().is_file() {
            return None;
        }

        let media_type =
            media_type_from_extension(content_file_path.extension()?.to_string_lossy().as_ref())?;
        let id =
            ContentId::from_str(content_file_path.file_stem()?.to_string_lossy().as_ref()).ok()?;

        Some(Content::new(
            id,
            media_type,
            format!("{}/{}.avif", self.content_url_base, id.as_ref())
                .parse()
                .unwrap(),
            format!("{}/{}.avif", self.thumbnail_url_base, id.as_ref())
                .parse()
                .unwrap(),
        ))
    }
}

impl ContentStorage for FileSystemContentStorage {
    fn get_content(&self, id: ContentId) -> Result<Content, Error> {
        let content_file_path = self.content_file_path(id);

        let exist = fs::exists(&content_file_path).map_err(|e| {
            Error::Internal(format!(
                "Failed to check existence of content file: {}",
                e.to_string()
            ))
        })?;
        if !exist {
            return Err(Error::ContentNotFound);
        }

        self.generate_content(&content_file_path)
            .ok_or(Error::ContentNotFound)
    }

    fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        let mut contents: Vec<Content> = fs::read_dir(&self.contents_directory)
            .map_err(|e| {
                Error::Internal(format!(
                    "Failed to read contents directory: {}",
                    e.to_string()
                ))
            })?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| self.generate_content(&entry.path()))
            .collect();

        contents.sort_by_key(|content| content.id().as_ref().clone());

        // 以下のようにしてIter<Content>を生成して、contents_after_cursor変数にバインド
        // _cursorがNoneの場合にはself.contentsをそのまま使う
        // 存在する場合には、idがcursorと同じ要素以降のみを返す
        let contents_after_cursor: Vec<Content> = match cursor {
            Some(cursor) => {
                let cursor_id =
                    ContentId::from_str(cursor.as_str()).map_err(|_| Error::InvalidCursor)?;

                let index = contents
                    .iter()
                    .position(|content| content.id().as_ref() == cursor_id.as_ref())
                    .ok_or(Error::InvalidCursor)?;

                contents[index..].to_vec()
            }
            None => contents,
        };

        Ok(if contents_after_cursor.len() <= limit as usize {
            (contents_after_cursor, None)
        } else {
            let next_cursor_id = contents_after_cursor[limit as usize].id();
            (
                contents_after_cursor[..limit as usize].to_vec(),
                Some(next_cursor_id.as_ref().to_string()),
            )
        })
    }
}

fn media_type_from_extension<T>(extension: T) -> Option<MediaType>
where
    T: AsRef<str>,
{
    match extension.as_ref() {
        "avif" => Some(MediaType::AVIF),
        _ => None,
    }
}

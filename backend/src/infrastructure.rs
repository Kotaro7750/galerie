use std::fs::{self};
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

use xmp_toolkit::{OpenFileOptions, XmpError, XmpFile, XmpMeta};

use crate::domain::tag::parse_xmp;
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
        self.contents_directory
            .clone()
            .join(format!("{}.xmp", id.as_ref()))
    }

    fn content_file_path(&self, id: ContentId) -> PathBuf {
        self.contents_directory
            .clone()
            .join(format!("{}.avif", id.as_ref()))
    }

    /// コンテンツディレクトリ配下から拡張子を除いたファイル名の重複を除いたユニークな一覧をイテレータとして返す
    fn list_unique_content_ids(&self) -> Result<impl Iterator<Item = ContentId>, io::Error> {
        Ok(fs::read_dir(&self.contents_directory)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_file() {
                    path.file_stem()
                        .and_then(|stem| stem.to_str())
                        .and_then(|stem_str| ContentId::from_str(stem_str).ok())
                } else {
                    None
                }
            })
            .collect::<std::collections::HashSet<ContentId>>()
            .into_iter())
    }

    /// Check if both content file and xmp file exist and are files for the given content id
    fn has_valid_content_pair(&self, id: ContentId) -> Result<bool, io::Error> {
        let content_file_path = self.content_file_path(id);
        let xmp_file_path = self.xmp_file_path(id);

        // Check if the content file and xmp file both exist
        if !fs::exists(&content_file_path)? || !fs::exists(&xmp_file_path)? {
            return Ok(false);
        }

        // Check if the content file and xmp file both are files
        if !fs::metadata(&content_file_path)?.file_type().is_file() {
            Ok(false)
        } else if !fs::metadata(&xmp_file_path)?.file_type().is_file() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Extract XMP metadata for the given content id
    fn extract_xmp_metadata(&self, id: ContentId) -> Result<Option<XmpMeta>, XmpError> {
        let xmp_file_path = self.xmp_file_path(id);

        let mut xmp_file = XmpFile::new()?;
        xmp_file.open_file(
            &xmp_file_path,
            OpenFileOptions::default().for_read().only_xmp(),
        )?;

        Ok(xmp_file.xmp())
    }

    /// Extract mediatype for the given content id
    fn extract_media_type(&self, id: ContentId) -> Option<MediaType> {
        let content_file_path = self.content_file_path(id);

        media_type_from_extension(content_file_path.extension()?.to_string_lossy().as_ref())
    }
}

impl ContentStorage for FileSystemContentStorage {
    fn get_content(&self, id: ContentId) -> Result<Content, Error> {
        if !self.has_valid_content_pair(id).map_err(|e| {
            Error::Internal(format!(
                "Failed to check existence of content pair: {}",
                e.to_string()
            ))
        })? {
            return Err(Error::ContentNotFound);
        }

        let metadata = self
            .extract_xmp_metadata(id)
            .map_err(|e| {
                Error::Internal(format!("Failed to extract XMP metadata: {}", e.to_string()))
            })?
            .ok_or(Error::ContentNotFound)?;
        let tag_parse_result = parse_xmp(metadata);

        let media_type = self.extract_media_type(id).ok_or(Error::ContentNotFound)?;

        Ok(Content::new(
            id,
            media_type,
            format!("{}/{}.avif", self.content_url_base, id.as_ref())
                .parse()
                .unwrap(),
            format!("{}/{}.avif", self.thumbnail_url_base, id.as_ref())
                .parse()
                .unwrap(),
            tag_parse_result.parsed().to_vec(),
            tag_parse_result.skipped().to_vec(),
        ))
    }

    fn list_contents(
        &self,
        limit: u64,
        cursor: Option<String>,
    ) -> Result<(Vec<Content>, Option<String>), Error> {
        let mut contents = self
            .list_unique_content_ids()
            .map_err(|e| {
                Error::Internal(format!(
                    "Failed to list unique content ids: {}",
                    e.to_string()
                ))
            })?
            .filter_map(|id| self.get_content(id).ok())
            .collect::<Vec<Content>>();

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

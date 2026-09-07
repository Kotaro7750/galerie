use std::collections::BinaryHeap;
use std::fs::{self};
use std::io;
use std::num::NonZeroU64;
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

    /// Construct an iterator over the valid content ids retrieved from xmp files in the contents directory
    fn xmp_content_id_iter(&self) -> Result<impl Iterator<Item = ContentId>, io::Error> {
        Ok(fs::read_dir(&self.contents_directory)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter(|file| file.path().extension().map_or(false, |ext| ext == "xmp"))
            .filter_map(|xmp_entry| {
                xmp_entry
                    .path()
                    .file_stem()
                    .and_then(|xmp_stem| xmp_stem.to_str())
                    .and_then(|xmp_stem| ContentId::from_str(xmp_stem).ok())
            }))
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
    fn scan_contents(
        &self,
        limit: NonZeroU64,
        cursor: Option<ContentId>,
    ) -> Result<(Vec<Content>, Option<ContentId>), Error> {
        let mut heap = BinaryHeap::<Content>::new();

        for id in self
            .xmp_content_id_iter()
            .map_err(|e| Error::Internal(e.to_string()))?
        {
            if let Some(cursor) = cursor {
                if id.as_ref() <= cursor.as_ref() {
                    continue;
                }
            }

            if !self
                .has_valid_content_pair(id)
                .ok()
                .map_or(false, |valid| valid)
            {
                continue;
            }

            if let Some(media_type) = self.extract_media_type(id) {
                if let Some(Some(metadata)) = self.extract_xmp_metadata(id).ok() {
                    let tag_parse_result = parse_xmp(metadata);

                    let content = Content::new(
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
                    );

                    if heap.len() as u64 >= limit.get() && content >= *heap.peek().unwrap() {
                        continue;
                    }

                    if limit.get() <= heap.len() as u64 {
                        heap.pop();
                    }

                    heap.push(content);
                }
            }
        }

        let scanned_contents = heap.into_sorted_vec();
        let next_cursor = if scanned_contents.len() as u64 == limit.get() {
            Some(scanned_contents.last().unwrap().id())
        } else {
            None
        };

        Ok((scanned_contents, next_cursor))
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

use std::str::FromStr;

use mime::Mime;
use url::Url;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) enum Error {
    ContentNotFound,
    InvalidCursor,
    Internal(String),
}

#[derive(Debug, Clone, Copy)]
/// ContentId is a valid UUID v4 that represents the unique identifier of a content.
pub(crate) struct ContentId(Uuid);

impl ContentId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for ContentId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|e| format!("Invalid UUID string: {}", e))?;
        uuid.try_into()
    }
}

impl TryFrom<Uuid> for ContentId {
    type Error = String;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        if value.get_version_num() == 4 && value.get_variant() == uuid::Variant::RFC4122 {
            Ok(Self(value))
        } else {
            Err(format!(
                "Invalid UUID version: expected v4, got v{}",
                value.get_version_num()
            ))
        }
    }
}

impl AsRef<Uuid> for ContentId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum MediaType {
    AVIF,
}

impl From<MediaType> for Mime {
    fn from(media_type: MediaType) -> Self {
        match media_type {
            MediaType::AVIF => "image/avif".parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Content {
    id: ContentId,
    media_type: MediaType,
    content_url: Url,
    thumbnail_url: Url,
}

impl Content {
    pub(crate) fn new(
        id: ContentId,
        media_type: MediaType,
        content_url: Url,
        thumbnail_url: Url,
    ) -> Self {
        Self {
            id,
            media_type,
            content_url,
            thumbnail_url,
        }
    }

    pub(crate) fn id(&self) -> ContentId {
        self.id
    }

    pub(crate) fn media_type(&self) -> MediaType {
        *(&self.media_type)
    }

    pub(crate) fn content_url(&self) -> &Url {
        &self.content_url
    }

    pub(crate) fn thumbnail_url(&self) -> &Url {
        &self.thumbnail_url
    }
}

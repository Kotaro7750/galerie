use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::tags::Tag;

/// Representation of a media file discovered on disk.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MediaFile {
    pub id: String,
    pub relative_path: String,
    pub media_type: MediaType,
    pub tags: Vec<Tag>,
    pub attributes: HashMap<String, String>,
    pub filesize: u64,
    pub dimensions: Option<Dimensions>,
    pub duration_ms: Option<u64>,
    pub thumbnail_path: Option<String>,
    pub hash: Option<String>,
    pub indexed_at: DateTime<Utc>,
}

/// Placeholder for image/video dimensions. Populated once metadata extraction lands.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// Supported media types. `Unknown` is used internally until richer detection ships.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Gif,
    Video,
    Audio,
    Pdf,
    Unknown,
}

impl Default for MediaType {
    fn default() -> Self {
        Self::Unknown
    }
}

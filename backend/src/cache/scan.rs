use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result, bail};
use chrono::Utc;
use tracing::instrument;
use walkdir::{DirEntry, WalkDir};

use crate::{
    media::{MediaFile, MediaType},
    tags::{Tag, TagKind, parse_filename_tokens},
};

/// Perform a single scan of the media root, returning discovered files.
#[instrument(skip(root), fields(media_root = %root.display()), err)]
pub fn scan_media(root: &Path) -> Result<Vec<MediaFile>> {
    if !root.exists() {
        anyhow::bail!(
            "media root '{}' does not exist",
            root.as_os_str().to_string_lossy()
        );
    }

    let mut files = Vec::new();
    let indexed_at = Utc::now();

    for entry in WalkDir::new(root).into_iter() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                tracing::warn!(error = %err, "failed to read directory entry");
                continue;
            }
        };

        let rel_display = entry
            .path()
            .strip_prefix(root)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| entry.path().display().to_string());

        if !entry.file_type().is_file() {
            continue;
        }

        match build_media_file(root, &entry, indexed_at, &rel_display) {
            Ok(media_file) => files.push(media_file),
            Err(err) => {
                tracing::warn!(path = %rel_display, error = ?err, "skipping media file due to error");
            }
        }
    }

    Ok(files)
}

#[instrument(skip(root, entry, indexed_at, rel_display), fields(path = %rel_display))]
fn build_media_file(
    root: &Path,
    entry: &DirEntry,
    indexed_at: chrono::DateTime<Utc>,
    rel_display: &str,
) -> Result<MediaFile> {
    let relative = entry
        .path()
        .strip_prefix(root)
        .context("entry not under media root")?;

    let relative_path = relative_to_string(relative);
    let metadata = entry.metadata().context("failed to read metadata")?;
    let filesize = metadata.len();
    let media_type = detect_media_type(entry.path());
    if matches!(media_type, MediaType::Unknown) {
        bail!("unsupported media type");
    }
    let stem = entry
        .path()
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default();
    let parse_result = parse_filename_tokens(stem);
    if !parse_result.invalid_tokens.is_empty() {
        tracing::warn!(
            path = %rel_display,
            invalid = ?parse_result.invalid_tokens,
            "ignored invalid tag tokens"
        );
    }
    let attributes = build_attributes_from_tags(&parse_result.tags);

    tracing::info!(path = %rel_display,"scanned media file {}", relative_path);

    let media_id = stable_id(relative);

    Ok(MediaFile {
        id: media_id.clone(),
        relative_path,
        media_type,
        tags: parse_result.tags,
        attributes,
        filesize,
        dimensions: None,
        duration_ms: None,
        thumbnail_path: Some(format!("/media/{media_id}/thumbnail")),
        hash: None,
        indexed_at,
    })
}

fn detect_media_type(path: &Path) -> MediaType {
    let Some(ext) = path.extension().and_then(|ext| ext.to_str()) else {
        return MediaType::Unknown;
    };

    match ext.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "webp" | "bmp" | "heic" | "tiff" => MediaType::Image,
        "gif" => MediaType::Gif,
        "mp4" | "mov" | "mkv" | "webm" | "avi" => MediaType::Video,
        "mp3" | "wav" | "flac" | "aac" | "ogg" => MediaType::Audio,
        "pdf" => MediaType::Pdf,
        _ => MediaType::Unknown,
    }
}

fn stable_id(relative: &Path) -> String {
    use sha1::{Digest, Sha1};

    let normalized = relative_to_string(relative);
    let mut hasher = Sha1::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn relative_to_string(path: &Path) -> String {
    let mut normalized = path.to_string_lossy().to_string();
    if std::path::MAIN_SEPARATOR != '/' {
        normalized = normalized.replace(std::path::MAIN_SEPARATOR, "/");
    }
    normalized
}

fn build_attributes_from_tags(tags: &[Tag]) -> HashMap<String, String> {
    let mut attributes = HashMap::new();
    for tag in tags {
        if matches!(tag.kind, TagKind::KeyValue) {
            if let Some(value) = &tag.value {
                attributes
                    .entry(tag.name.clone())
                    .or_insert_with(|| value.clone());
            }
        }
    }
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn scan_once_discovers_files() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        std::fs::create_dir_all(root.join("nested"))?;
        std::fs::write(root.join("nested/example.jpg"), b"hello")?;

        let files = scan_media(root)?;
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].relative_path, "nested/example.jpg");
        assert_eq!(files[0].media_type, MediaType::Image);
        Ok(())
    }

    #[test]
    fn scan_once_ignores_unknown_media() -> Result<()> {
        let dir = tempdir()?;
        let root = dir.path();
        std::fs::write(root.join("notes.txt"), b"not media")?;

        let files = scan_media(root)?;
        assert!(files.is_empty(), "unknown media types should be skipped");
        Ok(())
    }
}

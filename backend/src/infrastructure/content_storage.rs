use crate::domain::MediaType;

pub(crate) mod filesystem;
pub(crate) mod s3;

fn media_type_from_extension<T>(extension: T) -> Option<MediaType>
where
    T: AsRef<str>,
{
    match extension.as_ref() {
        "avif" => Some(MediaType::Avif),
        _ => None,
    }
}

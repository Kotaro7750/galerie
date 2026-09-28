use super::TAG_NAMESPACE_URI;
use crate::domain::tag::{Tag, TagSet};

use xmp_toolkit::{ToStringOptions, XmpMeta, XmpValue};

pub(crate) fn serialize_tags_to_xmp(tags: &TagSet) -> Result<String, xmp_toolkit::XmpError> {
    XmpMeta::register_namespace(TAG_NAMESPACE_URI, "galerie")?;

    let mut metadata = XmpMeta::new()?;

    let mut tags: Vec<_> = tags.as_ref().values().collect();
    tags.sort_by_key(|tag| tag.key());

    for tag in tags {
        let key = tag.key().as_ref();

        match tag {
            Tag::KeyOnly { .. } => {
                metadata.set_property(TAG_NAMESPACE_URI, key, &XmpValue::new(String::new()))?
            }
            Tag::Text { value, .. } => metadata.set_property(
                TAG_NAMESPACE_URI,
                key,
                &XmpValue::new(value.as_ref().to_string()),
            )?,
            Tag::Integer { value, .. } => metadata.set_property(
                TAG_NAMESPACE_URI,
                key,
                &XmpValue::new(value.as_ref().to_string()),
            )?,
            Tag::Real { value, .. } => metadata.set_property(
                TAG_NAMESPACE_URI,
                key,
                &XmpValue::new(value.as_ref().to_string()),
            )?,
            Tag::TextSet { values, .. } => {
                let values = values.iter().map(|v| v.as_ref().to_string()).collect();
                append_xmp_set(&mut metadata, key, values)?;
            }
            Tag::IntegerSet { values, .. } => {
                let values = values.iter().map(|v| v.as_ref().to_string()).collect();
                append_xmp_set(&mut metadata, key, values)?;
            }
            Tag::RealSet { values, .. } => {
                let values = values.iter().map(|v| v.as_ref().to_string()).collect();
                append_xmp_set(&mut metadata, key, values)?;
            }
        }
    }
    metadata.to_string_with_options(ToStringOptions::default().omit_packet_wrapper())
}

/// Append a set of values to an XMP metadata object as an array property.
/// When values is empty, an empty array property is created.
fn append_xmp_set(
    metadata: &mut XmpMeta,
    key: &str,
    mut values: Vec<String>,
) -> Result<(), xmp_toolkit::XmpError> {
    values.sort();

    // By default xmp_toolkit does not support empty arrays, so we need to create an empty array property manually.
    if values.is_empty() {
        return metadata.set_property(
            TAG_NAMESPACE_URI,
            key,
            &XmpValue::new(String::new()).set_is_array(true),
        );
    }

    let array = XmpValue::new(key.to_string()).set_is_array(true);
    for value in values {
        metadata.append_array_item(TAG_NAMESPACE_URI, &array, &XmpValue::new(value))?;
    }
    Ok(())
}

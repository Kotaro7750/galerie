use super::TAG_NAMESPACE_URI;
use crate::domain::tag::{Tag, TagSet};

use xmp_toolkit::{IterOptions, ToStringOptions, XmpMeta, XmpValue};

/// Create a new XMP metadata object with the given tags.
pub(crate) fn create_xmp_with_tags(tags: &TagSet) -> Result<String, xmp_toolkit::XmpError> {
    XmpMeta::register_namespace(TAG_NAMESPACE_URI, "galerie")?;

    let mut metadata = XmpMeta::new()?;
    write_tags(&mut metadata, tags)?;
    metadata.to_string_with_options(ToStringOptions::default().omit_packet_wrapper())
}

/// Modify the given XMP metadata object with the given tags.
/// All existing properties in the galerie namespace are removed and replaced with the new tags, but
/// any other properties in the XMP metadata are preserved.
pub(crate) fn modify_xmp_with_tags(
    xmp: &str,
    tags: &TagSet,
) -> Result<String, xmp_toolkit::XmpError> {
    XmpMeta::register_namespace(TAG_NAMESPACE_URI, "galerie")?;
    let mut metadata: XmpMeta = xmp.parse()?;
    let properties: Vec<String> = metadata
        .iter(
            IterOptions::default()
                .schema_ns(TAG_NAMESPACE_URI)
                .immediate_children_only(),
        )
        .map(|property| property.name)
        .collect();
    for property in properties {
        metadata.delete_property(TAG_NAMESPACE_URI, &property)?;
    }
    write_tags(&mut metadata, tags)?;
    metadata.to_string_with_options(ToStringOptions::default().omit_packet_wrapper())
}

/// Write the tags to the XMP metadata object.
fn write_tags(metadata: &mut XmpMeta, tags: &TagSet) -> Result<(), xmp_toolkit::XmpError> {
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
                append_xmp_set(metadata, key, values)?;
            }
            Tag::IntegerSet { values, .. } => {
                let values = values.iter().map(|v| v.as_ref().to_string()).collect();
                append_xmp_set(metadata, key, values)?;
            }
            Tag::RealSet { values, .. } => {
                let values = values.iter().map(|v| v.as_ref().to_string()).collect();
                append_xmp_set(metadata, key, values)?;
            }
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::{Tag, TagKey, TextTagValue};

    #[test]
    fn replace_tags_removes_all_galerie_properties_and_keeps_foreign_properties() {
        let mut original = XmpMeta::new().unwrap();
        XmpMeta::register_namespace(TAG_NAMESPACE_URI, "galerie").unwrap();
        XmpMeta::register_namespace("urn:other", "other").unwrap();
        original
            .set_property(TAG_NAMESPACE_URI, "old", &XmpValue::new("old".to_string()))
            .unwrap();
        original
            .set_property(
                TAG_NAMESPACE_URI,
                "invalid-key",
                &XmpValue::new("bad".to_string()),
            )
            .unwrap();
        original
            .set_property("urn:other", "creator", &XmpValue::new("Alice".to_string()))
            .unwrap();

        let original = original
            .to_string_with_options(ToStringOptions::default().omit_packet_wrapper())
            .unwrap();

        let tags = TagSet::new(&[Tag::Text {
            key: TagKey::new("new").unwrap(),
            value: TextTagValue::new("updated").unwrap(),
        }])
        .unwrap();

        let updated: XmpMeta = modify_xmp_with_tags(&original, &tags)
            .unwrap()
            .parse()
            .unwrap();

        // Check if old properties are removed and new properties are added
        assert!(updated.property(TAG_NAMESPACE_URI, "old").is_none());
        assert!(updated.property(TAG_NAMESPACE_URI, "invalid-key").is_none());
        assert_eq!(
            updated.property(TAG_NAMESPACE_URI, "new").unwrap().value,
            "updated"
        );
        // Check if foreign properties are preserved
        assert_eq!(
            updated.property("urn:other", "creator").unwrap().value,
            "Alice"
        );

        let empty = TagSet::new(&[]).unwrap();
        let cleared: XmpMeta = modify_xmp_with_tags(&original, &empty)
            .unwrap()
            .parse()
            .unwrap();
        // Check if all galerie properties are removed
        assert_eq!(
            cleared
                .iter(
                    IterOptions::default()
                        .schema_ns(TAG_NAMESPACE_URI)
                        .immediate_children_only()
                )
                .count(),
            0
        );
        // Check if foreign properties are preserved
        assert_eq!(
            cleared.property("urn:other", "creator").unwrap().value,
            "Alice"
        );
    }
}

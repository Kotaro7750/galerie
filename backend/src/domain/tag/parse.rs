use super::*;
use std::collections::HashSet;

#[derive(Debug, Error)]
pub(crate) enum ParseMetadataError {
    #[error("Failed to parse XMP XML: {0}")]
    InvalidXml(#[from] quick_xml::Error),
    #[error("Failed to parse XMP metadata: {0}")]
    InvalidXmp(#[from] xmp_toolkit::XmpError),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Validate tag uniqueness before XMP Toolkit discards duplicate properties.
pub(crate) fn parse_metadata(metadata_xmp: &str) -> Result<Metadata, ParseMetadataError> {
    const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
    const TAG_NS: &str = "galerie";

    #[derive(PartialEq)]
    enum ElementContext {
        Rdf,
        Description,
        Other,
    }

    let mut reader = NsReader::from_str(metadata_xmp);
    reader.config_mut().expand_empty_elements = true;
    // Keep only the ancestry needed to distinguish properties from array/structure members.
    let mut ancestors = Vec::new();
    let mut keys = HashSet::new();
    let mut duplicate_keys = HashSet::new();
    let mut register = |key: &str| {
        if !keys.insert(key.to_owned()) {
            duplicate_keys.insert(key.to_owned());
        }
    };
    loop {
        match reader
            .read_event()
            .map_err(ParseMetadataError::InvalidXml)?
        {
            Event::Start(element) => {
                let (namespace, local) = reader.resolver().resolve_element(element.name());
                let is_rdf =
                    namespace == ResolveResult::Bound(Namespace(RDF_NS)) && local.as_ref() == "RDF";
                let is_description = namespace == ResolveResult::Bound(Namespace(RDF_NS))
                    && local.as_ref() == "Description"
                    && ancestors.last() == Some(&ElementContext::Rdf);

                if ancestors.last() == Some(&ElementContext::Description)
                    && namespace == ResolveResult::Bound(Namespace(TAG_NS))
                {
                    register(local.as_ref());
                }
                if is_description {
                    for attribute in element.attributes() {
                        let attribute = attribute.map_err(quick_xml::Error::from)?;
                        let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
                        if namespace == ResolveResult::Bound(Namespace(TAG_NS)) {
                            register(local.as_ref());
                        }
                    }
                }
                ancestors.push(if is_rdf {
                    ElementContext::Rdf
                } else if is_description {
                    ElementContext::Description
                } else {
                    ElementContext::Other
                });
            }
            Event::End(_) => {
                ancestors.pop();
            }
            Event::Eof => break,
            _ => {}
        }
    }

    let metadata = metadata_xmp.parse::<XmpMeta>()?;
    parse_xmp(metadata, &duplicate_keys)
}

/// Parse and validate already decoded XMP metadata.
/// Passed `duplicate_keys` are skipped.
fn parse_xmp(
    metadata: XmpMeta,
    duplicate_keys: &HashSet<String>,
) -> Result<Metadata, ParseMetadataError> {
    let mut parsed = Vec::new();
    let mut skipped = duplicate_keys
        .iter()
        .map(|key| SkippedTag {
            key: key.clone(),
            reason: SkippedReason::DuplicateKey,
        })
        .collect::<Vec<_>>();

    for property in metadata.iter(
        IterOptions::default()
            .schema_ns("galerie")
            .immediate_children_only(),
    ) {
        let local_key = property.name.split_once(':').map_or("", |(_, key)| key);

        if duplicate_keys.contains(local_key) {
            continue;
        }

        match parse_xmp_property(&metadata, property) {
            Ok(tag) => parsed.push(tag),
            Err(skipped_tag) => skipped.push(skipped_tag),
        }
    }

    // Duplicate keys are already skipped, so we can safely collect the parsed tags into a HashMap.
    Ok(Metadata {
        parsed: TagSet::new(parsed.as_slice()).ok_or(ParseMetadataError::Internal(
            "Duplicate keys found after parsing, which should not happen.".to_string(),
        ))?,
        skipped: SkippedTagSet::new(skipped.as_slice()).ok_or(ParseMetadataError::Internal(
            "Duplicate skipped keys found after parsing, which should not happen.".to_string(),
        ))?,
    })
}

/// Parse a single XMP property into a Tag, or return a SkippedTag if it cannot be parsed as a valid Tag.
fn parse_xmp_property(metadata: &XmpMeta, property: XmpProperty) -> Result<Tag, SkippedTag> {
    // 1. Extract local key from the property name, removing any namespace prefix
    let local_key = property
        .name
        .as_str()
        // Remove the namespace prefix if it exists
        .split_once(":")
        .map_or("", |(_, local)| local)
        .to_string();

    // 2. Validate tag key
    let tag_key = match TagKey::new(local_key.as_str()) {
        Some(key) => key,
        None => {
            return Err(SkippedTag {
                key: local_key,
                reason: SkippedReason::InvalidKey,
            });
        }
    };

    // 3. Parse tag value
    // INFO: Currently, we don't support integer or real types, because we cannot determine the type of
    // the value without additional schema information.
    if property.value.is_array() && !property.value.is_ordered() {
        Ok(Tag::TextSet {
            key: tag_key,
            values: parse_set_tag_value(
                metadata
                    .property_array(&property.schema_ns, &property.name)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map_err(|err| SkippedTag {
                key: local_key,
                reason: err,
            })?,
        })
    } else if property.value.is_struct() || property.value.is_ordered() {
        Err(SkippedTag {
            key: local_key,
            reason: SkippedReason::UnsupportedValueType,
        })
    } else {
        let value = property.value.value.clone();

        if value.is_empty() {
            Ok(Tag::KeyOnly { key: tag_key })
        } else {
            Ok(Tag::Text {
                key: tag_key,
                value: TextTagValue::new(&value).ok_or(SkippedTag {
                    key: local_key,
                    reason: SkippedReason::InvalidValue,
                })?,
            })
        }
    }
}

fn parse_set_tag_value(
    values: &[XmpValue<String>],
) -> Result<HashSet<TextTagValue>, SkippedReason> {
    let mut values_set = HashSet::new();

    for value in values.iter().map(|v| v.value.as_str()) {
        if let Some(value) = TextTagValue::new(value) {
            if !values_set.insert(value) {
                return Err(SkippedReason::DuplicateSetValue);
            }
        } else {
            return Err(SkippedReason::InvalidValue);
        }
    }

    Ok(values_set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_tags_are_skipped() {
        let file_path = "test-fixture/duplicate_tags.xmp";
        let xml = std::fs::read_to_string(file_path).expect("Failed to read XMP file");
        let parse_result = parse_metadata(&xml);

        assert!(
            parse_result.is_ok(),
            "Failed to parse XMP XML: {:?}",
            parse_result.err()
        );
        assert_eq!(
            parse_result
                .unwrap()
                .skipped()
                .as_ref()
                .get("DuplicatedTag"),
            Some(&SkippedTag {
                key: "DuplicatedTag".to_string(),
                reason: SkippedReason::DuplicateKey,
            })
        );
    }

    #[test]
    fn test_parse_xmp() {
        let file_path = "test-fixture/parse_xmp.xmp";
        let xml = std::fs::read_to_string(file_path).expect("Failed to read XMP file");
        let parse_result = parse_metadata(&xml).expect("Failed to parse XMP XML");

        assert_eq!(
            parse_result.parsed,
            TagSet::new(
                vec![
                    Tag::TextSet {
                        key: TagKey::new("Array").unwrap(),
                        values: vec![
                            TextTagValue("あ".to_string()),
                            TextTagValue("い".to_string()),
                            TextTagValue("う".to_string())
                        ]
                        .into_iter()
                        .collect()
                    },
                    Tag::KeyOnly {
                        key: TagKey::new("BooleanTrue").unwrap()
                    },
                    Tag::Text {
                        key: TagKey::new("LiteralTrue").unwrap(),
                        value: TextTagValue::new("True").unwrap()
                    },
                    Tag::Text {
                        key: TagKey::new("LiteralFalse").unwrap(),
                        value: TextTagValue::new("False").unwrap()
                    },
                    Tag::Text {
                        key: TagKey::new("Integer").unwrap(),
                        value: TextTagValue::new("-100").unwrap()
                    },
                    Tag::TextSet {
                        key: TagKey::new("IntegerArray").unwrap(),
                        values: vec![
                            TextTagValue("-100".to_string()),
                            TextTagValue("0".to_string()),
                            TextTagValue("100".to_string())
                        ]
                        .into_iter()
                        .collect()
                    },
                    Tag::Text {
                        key: TagKey::new("Real").unwrap(),
                        value: TextTagValue::new("3.14").unwrap()
                    },
                    Tag::TextSet {
                        key: TagKey::new("RealArray").unwrap(),
                        values: vec![
                            TextTagValue("-2.5".to_string()),
                            TextTagValue("0.0".to_string()),
                            TextTagValue("3.14".to_string())
                        ]
                        .into_iter()
                        .collect()
                    },
                    Tag::Text {
                        key: TagKey::new("Text").unwrap(),
                        value: TextTagValue::new("これはテストです").unwrap()
                    },
                ]
                .as_slice()
            )
            .unwrap()
        );
        assert_eq!(
            parse_result.skipped,
            SkippedTagSet::new(
                vec![
                    SkippedTag {
                        key: "BooleanFalse".to_string(),
                        reason: SkippedReason::InvalidValue
                    },
                    SkippedTag {
                        key: "DuplicatedSet".to_string(),
                        reason: SkippedReason::DuplicateSetValue
                    },
                    SkippedTag {
                        key: "Invalid-Key".to_string(),
                        reason: SkippedReason::InvalidKey
                    },
                    SkippedTag {
                        key: "OrderedArray".to_string(),
                        reason: SkippedReason::UnsupportedValueType
                    },
                    SkippedTag {
                        key: "struct".to_string(),
                        reason: SkippedReason::UnsupportedValueType
                    },
                ]
                .as_slice()
            )
            .unwrap()
        );
    }

    #[test]
    fn sample_sidecars_preserve_key_only_and_invalid_tags() {
        for (file, key_only) in [
            ("00065786-f916-4e2c-85bc-3db5e4c0cb71", None),
            ("2a79b6b3-ffa5-4f8d-82b9-9e81d989c5f8", Some("animation")),
            ("ee375a99-75b9-4cc5-8bfb-f6b2e5c8505b", Some("transparent")),
        ] {
            let xml = std::fs::read_to_string(format!("../sample/{file}.xmp")).unwrap();
            let metadata = parse_metadata(&xml).unwrap();

            if let Some(key_only) = key_only {
                assert_eq!(
                    metadata.parsed().as_ref().get(&TagKey::new(key_only).unwrap()),
                    Some(&Tag::KeyOnly {
                        key: TagKey::new(key_only).unwrap(),
                    })
                );
            }
            assert_eq!(
                metadata.skipped().as_ref().get("disabled"),
                Some(&SkippedTag {
                    key: "disabled".to_string(),
                    reason: SkippedReason::InvalidValue,
                })
            );
        }
    }
}

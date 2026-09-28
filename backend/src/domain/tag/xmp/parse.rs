use crate::domain::tag::{Tag, TagKey, TagSet};
use crate::domain::{ContentDiagnostic, tag_schema::TagSchema};

use super::TAG_NAMESPACE_URI;
use quick_xml::NsReader;
use quick_xml::events::Event;
use quick_xml::name::{Namespace, ResolveResult};
use std::collections::HashSet;
use thiserror::Error;
use xmp_toolkit::{IterOptions, XmpMeta, XmpProperty};

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
pub(crate) fn parse_metadata(
    metadata_xmp: &str,
    tag_schema: &TagSchema,
) -> Result<(TagSet, HashSet<ContentDiagnostic>), ParseMetadataError> {
    const RDF_NS: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

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
                    && namespace == ResolveResult::Bound(Namespace(TAG_NAMESPACE_URI))
                {
                    register(local.as_ref());
                }
                if is_description {
                    for attribute in element.attributes() {
                        let attribute = attribute.map_err(quick_xml::Error::from)?;
                        let (namespace, local) = reader.resolver().resolve_attribute(attribute.key);
                        if namespace == ResolveResult::Bound(Namespace(TAG_NAMESPACE_URI)) {
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
    parse_xmp(metadata, tag_schema, &duplicate_keys)
}

/// Parse and validate already decoded XMP metadata.
/// Passed `duplicate_keys` are skipped.
fn parse_xmp(
    metadata: XmpMeta,
    tag_schema: &TagSchema,
    duplicate_keys: &HashSet<String>,
) -> Result<(TagSet, HashSet<ContentDiagnostic>), ParseMetadataError> {
    let mut parsed_tags = Vec::new();
    let mut diagnostics = duplicate_keys
        .iter()
        .map(|key| {
            if let Some(key) = TagKey::new(key) {
                ContentDiagnostic::DuplicateKey { key }
            } else {
                ContentDiagnostic::InvalidKey { key: key.clone() }
            }
        })
        .collect::<HashSet<_>>();

    for property in metadata.iter(
        IterOptions::default()
            .schema_ns(TAG_NAMESPACE_URI)
            .immediate_children_only(),
    ) {
        let local_key = strip_namespace_from_key(&property.name);

        if duplicate_keys.contains(&local_key) {
            continue;
        }

        match parse_xmp_property(&metadata, tag_schema, property) {
            Ok(tag) => parsed_tags.push(tag),
            Err(diagnostic) => {
                diagnostics.insert(diagnostic);
            }
        }
    }

    tag_schema
        .diagnose_tags_combination(&parsed_tags)
        .iter()
        .for_each(|diagnostic| {
            diagnostics.insert(diagnostic.clone());
        });

    // Duplicate keys are already skipped, so we can safely collect the parsed tags into a HashMap.
    Ok((
        TagSet::new(parsed_tags.as_slice()).ok_or(ParseMetadataError::Internal(
            "Duplicate keys found after parsing, which should not happen.".to_string(),
        ))?,
        diagnostics,
    ))
}

/// Parse a single XMP property into a Tag, or return a SkippedTag if it cannot be parsed as a valid Tag.
fn parse_xmp_property(
    metadata: &XmpMeta,
    tag_schema: &TagSchema,
    property: XmpProperty,
) -> Result<Tag, ContentDiagnostic> {
    // 1. Extract local key from the property name, removing any namespace prefix
    let local_key = strip_namespace_from_key(&property.name);

    // 2. Validate tag key
    let tag_key = match TagKey::new(local_key.as_str()) {
        Some(key) => key,
        None => {
            return Err(ContentDiagnostic::InvalidKey { key: local_key });
        }
    };

    // 3. Parse tag value
    if property.value.is_struct() || property.value.is_ordered() {
        Err(ContentDiagnostic::UnsupportedXmpValueType { key: tag_key })
    } else if property.value.is_array() && !property.value.is_ordered() {
        // For set tags, we need to detect duplicate values before normalize.
        let mut values_set = HashSet::new();

        for value in metadata
            .property_array(&property.schema_ns, &property.name)
            .collect::<Vec<_>>()
            .iter()
            .map(|v| v.value.as_str())
        {
            if !values_set.insert(value.to_string()) {
                return Err(ContentDiagnostic::DuplicateSetValue { key: tag_key });
            }
        }

        tag_schema.normalize_set_tag(tag_key, &values_set)
    } else {
        tag_schema.normalize_tag(tag_key, property.value.value.clone())
    }
}

/// Strip the namespace prefix from a key, if it exists.
/// For example, "galerie:TagName" becomes "TagName".
fn strip_namespace_from_key(key: &str) -> String {
    key.split_once(':').map_or("", |(_, key)| key).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tag::{IntegerTagValue, RealTagValue, TextTagValue};

    fn xmp(properties: &str) -> String {
        format!(
            r#"<x:xmpmeta xmlns:x="adobe:ns:meta/">
  <rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
    <rdf:Description rdf:about="" xmlns:galerie="galerie" xmlns:other="https://example.com/other/">
      {properties}
    </rdf:Description>
  </rdf:RDF>
</x:xmpmeta>"#
        )
    }

    fn schema(yaml: &str) -> TagSchema {
        serde_yaml::from_str(yaml).unwrap()
    }

    fn assert_rejected(properties: &str, expected: ContentDiagnostic) {
        let (parsed, diagnostics) =
            parse_metadata(&xmp(properties), &TagSchema::default()).unwrap();
        assert!(parsed.as_ref().is_empty());
        assert_eq!(diagnostics, [expected].into());
    }

    #[test]
    fn parse_metadata_excludes_duplicate_key() {
        assert_rejected(
            "<galerie:Tag>first</galerie:Tag><galerie:Tag>second</galerie:Tag>",
            ContentDiagnostic::DuplicateKey {
                key: TagKey::new("Tag").unwrap(),
            },
        );
    }

    #[test]
    fn parse_metadata_excludes_empty_array_item() {
        assert_rejected(
            "<galerie:Values><rdf:Bag><rdf:li/></rdf:Bag></galerie:Values>",
            ContentDiagnostic::UnparseableTagValue {
                key: TagKey::new("Values").unwrap(),
                definition: None,
            },
        );
    }

    #[test]
    fn parse_metadata_excludes_duplicate_text_set_value() {
        assert_rejected(
            "<galerie:Values><rdf:Bag><rdf:li>same</rdf:li><rdf:li>same</rdf:li></rdf:Bag></galerie:Values>",
            ContentDiagnostic::DuplicateSetValue {
                key: TagKey::new("Values").unwrap(),
            },
        );
    }

    #[test]
    fn parse_metadata_excludes_invalid_key() {
        assert_rejected(
            "<galerie:Invalid-Key>value</galerie:Invalid-Key>",
            ContentDiagnostic::InvalidKey {
                key: "Invalid-Key".to_string(),
            },
        );
    }

    #[test]
    fn parse_metadata_excludes_ordered_array() {
        assert_rejected(
            "<galerie:Values><rdf:Seq><rdf:li>value</rdf:li></rdf:Seq></galerie:Values>",
            ContentDiagnostic::UnsupportedXmpValueType {
                key: TagKey::new("Values").unwrap(),
            },
        );
    }

    #[test]
    fn parse_metadata_excludes_structured_property() {
        assert_rejected(
            "<galerie:Struct rdf:parseType=\"Resource\"><galerie:Name>value</galerie:Name></galerie:Struct>",
            ContentDiagnostic::UnsupportedXmpValueType {
                key: TagKey::new("Struct").unwrap(),
            },
        );
    }

    #[test]
    fn parse_metadata_preserves_key_only_and_invalid_tags_in_sample_sidecars() {
        for (file, key_only) in [
            ("00065786-f916-4e2c-85bc-3db5e4c0cb71", None),
            ("2a79b6b3-ffa5-4f8d-82b9-9e81d989c5f8", Some("animation")),
            ("ee375a99-75b9-4cc5-8bfb-f6b2e5c8505b", Some("transparent")),
        ] {
            let xml = std::fs::read_to_string(format!("../sample/{file}.xmp")).unwrap();
            let (parsed, diagnostics) = parse_metadata(&xml, &TagSchema::default()).unwrap();

            if let Some(key_only) = key_only {
                assert_eq!(
                    parsed.as_ref().get(&TagKey::new(key_only).unwrap()),
                    Some(&Tag::KeyOnly {
                        key: TagKey::new(key_only).unwrap(),
                    })
                );
            }
            assert!(
                diagnostics.contains(&ContentDiagnostic::UnparseableTagValue {
                    key: TagKey::new("disabled").unwrap(),
                    definition: None,
                })
            );
        }
    }

    #[test]
    fn parse_metadata_normalizes_all_defined_tag_types() {
        let tag_schema = schema(
            r#"version: '0'
allow_additional_tags: false
optional:
  Flag: { type: key_only }
  Label: { type: text }
  Count: { type: integer }
  Rating: { type: real }
  Labels: { type: text_set }
  Counts: { type: integer_set }
  Ratings: { type: real_set }
"#,
        );
        let xml = xmp(r#"<galerie:Flag/>
<galerie:Label>hello</galerie:Label>
<galerie:Count>42</galerie:Count>
<galerie:Rating>1.5</galerie:Rating>
<galerie:Labels><rdf:Bag><rdf:li>a</rdf:li><rdf:li>b</rdf:li></rdf:Bag></galerie:Labels>
<galerie:Counts><rdf:Bag><rdf:li>1</rdf:li><rdf:li>2</rdf:li></rdf:Bag></galerie:Counts>
<galerie:Ratings><rdf:Bag><rdf:li>1.5</rdf:li><rdf:li>2.5</rdf:li></rdf:Bag></galerie:Ratings>"#);

        let (parsed, diagnostics) = parse_metadata(&xml, &tag_schema).unwrap();
        let expected = TagSet::new(&[
            Tag::KeyOnly {
                key: TagKey::new("Flag").unwrap(),
            },
            Tag::Text {
                key: TagKey::new("Label").unwrap(),
                value: TextTagValue::new("hello").unwrap(),
            },
            Tag::Integer {
                key: TagKey::new("Count").unwrap(),
                value: IntegerTagValue::new(42).unwrap(),
            },
            Tag::Real {
                key: TagKey::new("Rating").unwrap(),
                value: RealTagValue::new(1.5).unwrap(),
            },
            Tag::TextSet {
                key: TagKey::new("Labels").unwrap(),
                values: [
                    TextTagValue::new("a").unwrap(),
                    TextTagValue::new("b").unwrap(),
                ]
                .into(),
            },
            Tag::IntegerSet {
                key: TagKey::new("Counts").unwrap(),
                values: [
                    IntegerTagValue::new(1).unwrap(),
                    IntegerTagValue::new(2).unwrap(),
                ]
                .into(),
            },
            Tag::RealSet {
                key: TagKey::new("Ratings").unwrap(),
                values: [
                    RealTagValue::new(1.5).unwrap(),
                    RealTagValue::new(2.5).unwrap(),
                ]
                .into(),
            },
        ])
        .unwrap();
        assert_eq!(parsed, expected);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn parse_metadata_reports_missing_required_tag_when_value_is_unparseable() {
        let tag_schema = schema(
            "version: '0'\nallow_additional_tags: false\nrequired:\n  Count: { type: integer }\n",
        );
        let xml = xmp("<galerie:Count>bad</galerie:Count>");

        let (parsed, diagnostics) = parse_metadata(&xml, &tag_schema).unwrap();
        assert!(parsed.as_ref().is_empty());
        let definition =
            crate::domain::tag_schema::TagDefinition::Integer(serde_yaml::from_str("{}").unwrap());
        assert_eq!(
            diagnostics,
            [
                ContentDiagnostic::UnparseableTagValue {
                    key: TagKey::new("Count").unwrap(),
                    definition: Some(definition.clone()),
                },
                ContentDiagnostic::MissingRequiredTag {
                    key: TagKey::new("Count").unwrap(),
                    definition,
                },
            ]
            .into()
        );
    }

    #[test]
    fn parse_metadata_ignores_same_named_properties_in_other_namespace() {
        let xml = xmp("<galerie:Tag>value</galerie:Tag><other:Tag>other</other:Tag>");

        let (parsed, diagnostics) = parse_metadata(&xml, &TagSchema::default()).unwrap();
        assert_eq!(
            parsed,
            TagSet::new(&[Tag::Text {
                key: TagKey::new("Tag").unwrap(),
                value: TextTagValue::new("value").unwrap(),
            }])
            .unwrap()
        );
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn parse_metadata_does_not_treat_nested_property_as_duplicate_key() {
        let xml = xmp(
            r#"<galerie:Outer rdf:parseType="Resource"><galerie:Inner>nested</galerie:Inner></galerie:Outer>
<galerie:Inner>top</galerie:Inner>"#,
        );

        let (parsed, diagnostics) = parse_metadata(&xml, &TagSchema::default()).unwrap();
        assert_eq!(
            parsed,
            TagSet::new(&[Tag::Text {
                key: TagKey::new("Inner").unwrap(),
                value: TextTagValue::new("top").unwrap(),
            }])
            .unwrap()
        );
        assert_eq!(
            diagnostics,
            [ContentDiagnostic::UnsupportedXmpValueType {
                key: TagKey::new("Outer").unwrap(),
            }]
            .into()
        );
    }

    #[test]
    fn parse_metadata_rejects_malformed_xml() {
        assert!(parse_metadata("<rdf:RDF", &TagSchema::default()).is_err());
    }
}

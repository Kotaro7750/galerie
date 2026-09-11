use std::collections::{BTreeSet, HashMap, HashSet};

use quick_xml::{
    events::Event,
    name::{Namespace, ResolveResult},
    reader::NsReader,
};
use thiserror::Error;
use unicode_xid::UnicodeXID;
use xmp_toolkit::{IterOptions, XmpMeta, XmpProperty, XmpValue};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tag {
    KeyOnly {
        key: TagKey,
    },
    Text {
        key: TagKey,
        value: TextTagValue,
    },
    #[allow(dead_code)]
    Integer {
        key: TagKey,
        value: IntegerTagValue,
    },
    #[allow(dead_code)]
    Real {
        key: TagKey,
        value: RealTagValue,
    },
    TextSet {
        key: TagKey,
        values: HashSet<TextTagValue>,
    },
    #[allow(dead_code)]
    IntegerSet {
        key: TagKey,
        values: HashSet<IntegerTagValue>,
    },
    #[allow(dead_code)]
    RealSet {
        key: TagKey,
        values: BTreeSet<RealTagValue>, // Due to floating point comparison issues, we use BTreeSet instead of HashSet for RealTagValue
    },
}

impl Tag {
    pub(crate) fn key(&self) -> &TagKey {
        match self {
            Tag::KeyOnly { key }
            | Tag::Text { key, .. }
            | Tag::Integer { key, .. }
            | Tag::Real { key, .. }
            | Tag::TextSet { key, .. }
            | Tag::IntegerSet { key, .. }
            | Tag::RealSet { key, .. } => key,
        }
    }

    /// Check if the tag contains the specified text value.
    pub(crate) fn contain_text_value(&self, text_value: &TextTagValue) -> bool {
        match self {
            Self::Text { key: _, value } => value == text_value,
            Self::TextSet { key: _, values } => values.contains(text_value),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TagKey {
    key: String,
}

impl TagKey {
    /// Constructs a new `TagKey` if the provided key is valid according to the specified rules.
    pub(crate) fn new(key: &str) -> Option<Self> {
        if Self::is_valid_for_key(key) {
            Some(Self {
                key: key.to_string(),
            })
        } else {
            None
        }
    }

    /// Check if the provided key is valid according to the specified rules.
    fn is_valid_for_key(key: &str) -> bool {
        if !unicode_normalization::is_nfkc(key) {
            return false;
        }

        let char_count = key.chars().count();
        if !(1..=64).contains(&char_count) {
            return false;
        }

        for (i, c) in key.chars().enumerate() {
            if (i == 0 && !c.is_xid_start()) || (i != 0 && !c.is_xid_continue()) {
                return false;
            }
        }

        true
    }
}

impl AsRef<str> for TagKey {
    fn as_ref(&self) -> &str {
        &self.key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TextTagValue(String);

impl TextTagValue {
    pub(crate) fn new(value: &str) -> Option<Self> {
        if unicode_normalization::is_nfc(value) {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }
}

impl AsRef<str> for TextTagValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct IntegerTagValue(i64);

impl IntegerTagValue {
    #[allow(dead_code)]
    fn new(value: i64) -> Option<Self> {
        if -(2_i64.pow(53) - 1) <= value && value < 2_i64.pow(53) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl AsRef<i64> for IntegerTagValue {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RealTagValue(f64);

impl AsRef<f64> for RealTagValue {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Metadata {
    parsed: TagSet,
    skipped: SkippedTagSet,
}

impl Metadata {
    pub(crate) fn parsed(&self) -> &TagSet {
        &self.parsed
    }

    pub(crate) fn skipped(&self) -> &SkippedTagSet {
        &self.skipped
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a set of tags, ensuring uniqueness of tag keys.
pub(crate) struct TagSet {
    tags: HashMap<TagKey, Tag>,
}

impl TagSet {
    pub(crate) fn new(tags: &[Tag]) -> Option<Self> {
        // Check for duplicate keys
        let mut set = HashSet::new();
        for tag in tags {
            if !set.insert(tag.key().clone()) {
                return None;
            }
        }

        Some(Self {
            tags: tags
                .iter()
                .map(|tag| (tag.key().clone(), tag.clone()))
                .collect(),
        })
    }
}

impl AsRef<HashMap<TagKey, Tag>> for TagSet {
    fn as_ref(&self) -> &HashMap<TagKey, Tag> {
        &self.tags
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a tag that was skipped during parsing, along with the reason for skipping.
pub(crate) struct SkippedTag {
    key: String,
    reason: SkippedReason,
}

impl SkippedTag {
    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn reason(&self) -> &SkippedReason {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SkippedReason {
    InvalidKey,
    DuplicateKey,
    UnsupportedValueType,
    InvalidValue,
    DuplicateSetValue,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a set of skipped tags, ensuring uniqueness of tag keys.
pub(crate) struct SkippedTagSet {
    tags: HashMap<String, SkippedTag>,
}

impl SkippedTagSet {
    pub(crate) fn new(tags: &[SkippedTag]) -> Option<Self> {
        let mut set = HashSet::new();
        for tag in tags {
            if !set.insert(tag.key.clone()) {
                return None;
            }
        }

        Some(Self {
            tags: tags
                .iter()
                .map(|tag| (tag.key.clone(), tag.clone()))
                .collect(),
        })
    }
}

impl AsRef<HashMap<String, SkippedTag>> for SkippedTagSet {
    fn as_ref(&self) -> &HashMap<String, SkippedTag> {
        &self.tags
    }
}

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

        if value == "False" {
            Err(SkippedTag {
                key: local_key,
                reason: SkippedReason::InvalidValue,
            })
        } else if value == "True" {
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
    use unicode_normalization::UnicodeNormalization;

    #[test]
    fn test_tag_key_validation_empty_key() {
        assert!(!TagKey::is_valid_for_key(""));
    }

    #[test]
    fn test_tag_key_validation_short_key() {
        assert!(TagKey::is_valid_for_key("a"));
    }

    #[test]
    fn test_tag_key_validation_long_key() {
        assert!(TagKey::is_valid_for_key("a".repeat(64).as_str()));
    }

    #[test]
    fn test_tag_key_validation_too_long_key() {
        assert!(!TagKey::is_valid_for_key("a".repeat(65).as_str()));
    }

    #[test]
    fn test_tag_key_validation_not_start_with_xid_start_key() {
        assert!(!TagKey::is_valid_for_key("1key"));
    }

    #[test]
    fn test_tag_key_validation_contain_xid_continue_key() {
        assert!(!TagKey::is_valid_for_key("key!"));
    }

    #[test]
    fn test_tag_key_validation_not_nkfc_valid_key() {
        // Decomposed character is not valid
        assert!(!TagKey::is_valid_for_key(
            "が".nfd().collect::<String>().as_str()
        ));

        // Non canonical character is not valid
        assert!(!TagKey::is_valid_for_key(
            "あ①1１".nfd().collect::<String>().as_str()
        ));
    }

    #[test]
    fn text_text_tag_value() {
        assert!(TextTagValue::new("が".nfd().collect::<String>().as_str()).is_none(),);
        assert!(TextTagValue::new("が".nfc().collect::<String>().as_str()).is_some(),);
    }

    #[test]
    fn text_integer_tag_value_range() {
        assert!(IntegerTagValue::new(-(2_i64.pow(53) - 1)).is_some());
        assert!(IntegerTagValue::new(2_i64.pow(53) - 1).is_some());
        assert!(IntegerTagValue::new(-(2_i64.pow(53))).is_none());
        assert!(IntegerTagValue::new(2_i64.pow(53)).is_none());
    }

    #[test]
    fn test_tag_key_validation_pass_valid_key() {
        assert!(TagKey::is_valid_for_key("有効なTag_Keyの1つです"));
    }

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
}

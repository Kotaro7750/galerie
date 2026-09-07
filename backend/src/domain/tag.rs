use unicode_xid::UnicodeXID;
use xmp_toolkit::{IterOptions, XmpMeta, XmpProperty};

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
    // TODO: VecではなくHashSetにした方がよい
    TextSet {
        key: TagKey,
        values: Vec<TextTagValue>,
    },
    #[allow(dead_code)]
    IntegerSet {
        key: TagKey,
        values: Vec<IntegerTagValue>,
    },
    #[allow(dead_code)]
    RealSet {
        key: TagKey,
        values: Vec<RealTagValue>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TagKey {
    key: String,
}

impl TagKey {
    /// Constructs a new `TagKey` if the provided key is valid according to the specified rules.
    fn new(key: &str) -> Option<Self> {
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TextTagValue(String);

impl TextTagValue {
    fn new(value: &str) -> Option<Self> {
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParseResult {
    parsed: Vec<Tag>,
    skipped: Vec<SkippedTag>,
}

impl ParseResult {
    pub(crate) fn parsed(&self) -> &[Tag] {
        &self.parsed
    }

    pub(crate) fn skipped(&self) -> &[SkippedTag] {
        &self.skipped
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    UnsupportedValueType,
    InvalidValue,
}

/// Parse and validate XMP metadata into Tags
pub(crate) fn parse_xmp(metadata: XmpMeta) -> ParseResult {
    let mut parsed = Vec::new();
    let mut skipped = Vec::new();

    for property in metadata.iter(
        IterOptions::default()
            .schema_ns("galerie")
            .immediate_children_only(),
    ) {
        match parse_xmp_property(&metadata, property) {
            Ok(tag) => parsed.push(tag),
            Err(skipped_tag) => skipped.push(skipped_tag),
        }
    }

    ParseResult { parsed, skipped }
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
            values: metadata
                .property_array(&property.schema_ns, &property.name)
                .map(|item| TextTagValue::new(&item.value))
                .collect::<Option<Vec<_>>>()
                .ok_or(SkippedTag {
                    key: local_key,
                    reason: SkippedReason::InvalidValue,
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

#[cfg(test)]
mod tests {
    use unicode_normalization::UnicodeNormalization;
    use xmp_toolkit::{OpenFileOptions, XmpFile};

    use super::*;

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
    fn test_parse_xmp() {
        let mut xmp_file = XmpFile::new().expect("Failed to create XmpFile");
        xmp_file
            .open_file(
                "test-fixture/parse_xmp.xmp",
                OpenFileOptions::default().for_read(),
            )
            .expect("Failed to open XMP file");

        let metadata = xmp_file.xmp().expect("Failed to get XMP metadata");

        let parse_result = parse_xmp(metadata);

        assert_eq!(
            parse_result.parsed,
            vec![
                Tag::KeyOnly {
                    key: TagKey::new("BooleanTrue").unwrap()
                },
                Tag::Text {
                    key: TagKey::new("Integer").unwrap(),
                    value: TextTagValue::new("-100").unwrap()
                },
                Tag::Text {
                    key: TagKey::new("Real").unwrap(),
                    value: TextTagValue::new("3.14").unwrap()
                },
                Tag::Text {
                    key: TagKey::new("Text").unwrap(),
                    value: TextTagValue::new("これはテストです").unwrap()
                },
                Tag::TextSet {
                    key: TagKey::new("Array").unwrap(),
                    values: vec![
                        TextTagValue("あ".to_string()),
                        TextTagValue("い".to_string()),
                        TextTagValue("う".to_string())
                    ]
                },
                Tag::TextSet {
                    key: TagKey::new("IntegerArray").unwrap(),
                    values: vec![
                        TextTagValue("-100".to_string()),
                        TextTagValue("0".to_string()),
                        TextTagValue("100".to_string())
                    ]
                },
                Tag::TextSet {
                    key: TagKey::new("RealArray").unwrap(),
                    values: vec![
                        TextTagValue("-2.5".to_string()),
                        TextTagValue("0.0".to_string()),
                        TextTagValue("3.14".to_string())
                    ]
                },
            ]
        );
        assert_eq!(
            parse_result.skipped,
            vec![
                SkippedTag {
                    key: "BooleanFalse".to_string(),
                    reason: SkippedReason::InvalidValue
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
        );
    }
}

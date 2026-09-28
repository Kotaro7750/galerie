use std::collections::{HashMap, HashSet};

use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use unicode_xid::UnicodeXID;

pub(crate) mod xmp;

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
        values: HashSet<RealTagValue>,
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

    /// Check if the tag contains the specified integer value.
    pub(crate) fn contain_integer_value(&self, integer_value: &IntegerTagValue) -> bool {
        match self {
            Self::Integer { key: _, value } => value == integer_value,
            Self::IntegerSet { key: _, values } => values.contains(integer_value),
            _ => false,
        }
    }

    /// Check if the tag contains the specified real value.
    pub(crate) fn contain_real_value(&self, real_value: &RealTagValue) -> bool {
        match self {
            Self::Real { key: _, value } => value == real_value,
            Self::RealSet { key: _, values } => values.contains(real_value),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
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

impl TryFrom<String> for TagKey {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::is_valid_for_key(&value) {
            Ok(Self { key: value })
        } else {
            Err(format!("Invalid text tag value: {}", value))
        }
    }
}

impl From<TagKey> for String {
    fn from(value: TagKey) -> Self {
        value.key
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, PartialOrd, Ord)]
#[serde(try_from = "String", into = "String")]
pub(crate) struct TextTagValue(String);

impl TextTagValue {
    pub(crate) fn new(value: &str) -> Option<Self> {
        if (1..=255).contains(&value.chars().count()) && unicode_normalization::is_nfc(value) {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }
}

impl TryFrom<String> for TextTagValue {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::new(&value).is_some() {
            Ok(Self(value))
        } else {
            Err(format!("Invalid tag key: {}", value))
        }
    }
}

impl AsRef<str> for TextTagValue {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<TextTagValue> for String {
    fn from(value: TextTagValue) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(try_from = "i64", into = "i64")]
pub(crate) struct IntegerTagValue(i64);

impl IntegerTagValue {
    pub(crate) fn new(value: i64) -> Option<Self> {
        if -(2_i64.pow(53) - 1) <= value && value < 2_i64.pow(53) {
            Some(Self(value))
        } else {
            None
        }
    }
}

impl TryFrom<i64> for IntegerTagValue {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match Self::new(value) {
            Some(num) => Ok(num),
            None => Err(format!("Invalid integer value: {}", value)),
        }
    }
}

impl From<IntegerTagValue> for i64 {
    fn from(value: IntegerTagValue) -> Self {
        value.0
    }
}

impl AsRef<i64> for IntegerTagValue {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(try_from = "f64", into = "f64")]
pub(crate) struct RealTagValue(NotNan<f64>);

impl RealTagValue {
    pub(crate) fn new(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }

        NotNan::new(value).ok().map(Self)
    }
}

impl AsRef<f64> for RealTagValue {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl TryFrom<f64> for RealTagValue {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match Self::new(value) {
            Some(num) => Ok(num),
            None => Err(format!("Invalid real value: {}", value)),
        }
    }
}

impl From<RealTagValue> for f64 {
    fn from(value: RealTagValue) -> Self {
        *value.0.as_ref()
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
        assert!(TextTagValue::new("").is_none());
        assert!(TextTagValue::new("が".nfd().collect::<String>().as_str()).is_none(),);
        assert!(TextTagValue::new("が".nfc().collect::<String>().as_str()).is_some(),);
    }

    #[test]
    fn text_tag_value_length_limit() {
        assert!(TextTagValue::new(&"あ".repeat(u8::MAX as usize)).is_some());
        assert!(TextTagValue::new(&"あ".repeat(u8::MAX as usize + 1)).is_none());
    }

    #[test]
    fn text_integer_tag_value_range() {
        assert!(IntegerTagValue::new(-(2_i64.pow(53) - 1)).is_some());
        assert!(IntegerTagValue::new(2_i64.pow(53) - 1).is_some());
        assert!(IntegerTagValue::new(-(2_i64.pow(53))).is_none());
        assert!(IntegerTagValue::new(2_i64.pow(53)).is_none());
    }

    #[test]
    fn real_tag_value_requires_finite_number() {
        assert!(RealTagValue::new(f64::MAX).is_some());
        assert!(RealTagValue::new(f64::MIN).is_some());
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(RealTagValue::new(value).is_none());
            assert!(RealTagValue::try_from(value).is_err());
        }
    }

    #[test]
    fn test_tag_key_validation_pass_valid_key() {
        assert!(TagKey::is_valid_for_key("有効なTag_Keyの1つです"));
    }
}

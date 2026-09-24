use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::domain::tag::{IntegerTagValue, RealTagValue, TagKey, TextTagValue};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "version")]
/// Represents the schema for tags in the system
pub(crate) enum TagSchema {
    #[serde(rename = "0")]
    V0(TagSchemaV0),
}

impl TagSchema {
    pub(crate) fn validate_format(&self) -> Result<(), String> {
        match self {
            TagSchema::V0(schema) => schema.validate_format(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
/// Represents the tag schema version 0
pub(crate) struct TagSchemaV0 {
    /// When true, allows additional tags not defined in this schema.
    /// When false, only tags defined in this schema are allowed.
    allow_additional_tags: bool,
    required: Option<HashMap<TagKey, TagDefinition>>,
    optional: Option<HashMap<TagKey, TagDefinition>>,
}

impl TagSchemaV0 {
    fn validate_format(&self) -> Result<(), String> {
        if let Some(required) = &self.required {
            for (key, def) in required {
                def.validate_format().map_err(|e| {
                    format!(
                        "Invalid definition for required tag '{}': {}",
                        key.as_ref(),
                        e
                    )
                })?;
            }
        }

        if let Some(optional) = &self.optional {
            for (key, def) in optional {
                def.validate_format().map_err(|e| {
                    format!(
                        "Invalid definition for optional tag '{}': {}",
                        key.as_ref(),
                        e
                    )
                })?;
            }
        }

        if let (Some(required), Some(optional)) = (&self.required, &self.optional)
            && let Some(key) = required.keys().find(|key| optional.contains_key(*key))
        {
            return Err(format!(
                "tag '{}' cannot be both required and optional",
                key.as_ref()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum TagDefinition {
    KeyOnly,
    Text(TextTagDefinition),
    Integer(IntegerTagDefinition),
    Real(RealTagDefinition),
    TextSet(TextTagDefinition),
    IntegerSet(IntegerTagDefinition),
    RealSet(RealTagDefinition),
}

impl TagDefinition {
    fn validate_format(&self) -> Result<(), String> {
        match self {
            TagDefinition::KeyOnly => Ok(()),
            TagDefinition::Text(def) => def.validate_format(),
            TagDefinition::Integer(def) => def.validate_format(),
            TagDefinition::Real(def) => def.validate_format(),
            TagDefinition::TextSet(def) => def.validate_format(),
            TagDefinition::IntegerSet(def) => def.validate_format(),
            TagDefinition::RealSet(def) => def.validate_format(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TextTagDefinition {
    /// Minimum/maximum length of the text value in UTF-8 characters.
    /// If not specified, there is no limit.
    /// Exclusive with `allowed_values`.
    min_length: Option<usize>,
    max_length: Option<usize>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min_length`/`max_length`.
    allowed_values: Option<HashSet<TextTagValue>>,
}

impl TextTagDefinition {
    fn validate_format(&self) -> Result<(), String> {
        if self.allowed_values.is_some() && (self.min_length.is_some() || self.max_length.is_some())
        {
            return Err("allowed_values is exclusive with min_length/max_length".to_string());
        }

        if let Some(min_length) = self.min_length
            && let Some(max_length) = self.max_length
            && min_length > max_length
        {
            return Err("min_length cannot be greater than max_length".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct IntegerTagDefinition {
    /// Minimum/maximum value, if not specified, there is no limitation.
    /// Exclusive with `allowed_values`.
    min: Option<IntegerTagValue>,
    max: Option<IntegerTagValue>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min`/`max`.
    allowed_values: Option<HashSet<IntegerTagValue>>,
}

impl IntegerTagDefinition {
    fn validate_format(&self) -> Result<(), String> {
        if self.allowed_values.is_some() && (self.min.is_some() || self.max.is_some()) {
            return Err("allowed_values is exclusive with min/max".to_string());
        }

        if let Some(min) = &self.min
            && let Some(max) = &self.max
            && min > max
        {
            return Err("min cannot be greater than max".to_string());
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct RealTagDefinition {
    /// Minimum/maximum value, if not specified, there is no limitation.
    /// Exclusive with `allowed_values`.
    min: Option<RealTagValue>,
    max: Option<RealTagValue>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min`/`max`.
    allowed_values: Option<Vec<RealTagValue>>,
}

impl RealTagDefinition {
    fn validate_format(&self) -> Result<(), String> {
        if self.allowed_values.is_some() && (self.min.is_some() || self.max.is_some()) {
            return Err("allowed_values is exclusive with min/max".to_string());
        }

        if let Some(min) = &self.min
            && let Some(max) = &self.max
            && min > max
        {
            return Err("min cannot be greater than max".to_string());
        }

        if let Some(allowed_values) = &self.allowed_values {
            let mut cloned = allowed_values.clone();
            cloned.sort_by(|a, b| a.as_ref().total_cmp(b.as_ref()));
            if cloned.windows(2).any(|w| w[0] == w[1]) {
                return Err("allowed_values contains duplicate values".to_string());
            }
        }
        Ok(())
    }
}

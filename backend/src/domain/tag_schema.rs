use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    num::NonZeroU8,
};

use crate::domain::{
    ContentDiagnostic,
    search_condition::{TagPredicate, Term},
    tag::{IntegerTagValue, RealTagValue, Tag, TagKey, TextTagValue},
};

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    pub(crate) fn is_tag_key_allowed(&self, key: &TagKey) -> bool {
        match self {
            TagSchema::V0(schema) => schema.is_tag_key_allowed(key),
        }
    }

    /// Normalizes a single key value parir into a tag based on the schema's rules.
    pub(crate) fn normalize_tag(
        &self,
        key: TagKey,
        value: String,
    ) -> Result<Tag, ContentDiagnostic> {
        match self {
            TagSchema::V0(schema) => schema.normalize_tag(&key, &value),
        }
    }

    /// Same as `normalize_tag` but for a set of values.
    pub(crate) fn normalize_set_tag(
        &self,
        key: TagKey,
        values: &HashSet<String>,
    ) -> Result<Tag, ContentDiagnostic> {
        match self {
            TagSchema::V0(schema) => schema.normalize_set_tag(&key, values),
        }
    }

    /// Diagnoses a set of tags against the schema, returning any diagnostics found.
    /// This method assumes that each of the tags has already been normalized and validated against the schema.
    /// So, this method is mainly for checking combination of tags is conformant.
    pub(crate) fn diagnose_tags_combination(&self, tags: &[Tag]) -> HashSet<ContentDiagnostic> {
        match self {
            TagSchema::V0(schema) => schema.diagnose_tags_combination(tags),
        }
    }

    /// Constructs a search term based on the schema's rules for a given tag key and candidate values.
    pub(crate) fn construct_search_term(
        &self,
        key: &TagKey,
        candidates: &[String],
    ) -> Option<Term> {
        match self {
            TagSchema::V0(schema) => schema.construct_search_term(key, candidates),
        }
    }
}

impl Default for TagSchema {
    fn default() -> Self {
        TagSchema::V0(TagSchemaV0::default())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

    /// Checks if a given tag key is allowed based on the schema's rules.
    fn is_tag_key_allowed(&self, key: &TagKey) -> bool {
        if self.allow_additional_tags {
            true
        } else {
            self.required.as_ref().is_some_and(|r| r.contains_key(key))
                || self.optional.as_ref().is_some_and(|o| o.contains_key(key))
        }
    }

    /// Retrieves the tag definition for a given key, if it exists in the schema.
    fn tag_definition_for_key(&self, key: &TagKey) -> Option<&TagDefinition> {
        self.required
            .as_ref()
            .and_then(|r| r.get(key))
            .or_else(|| self.optional.as_ref().and_then(|o| o.get(key)))
    }

    /// Normalizes a single key value parir into a tag based on the schema's rules.
    fn normalize_tag(&self, key: &TagKey, value: &str) -> Result<Tag, ContentDiagnostic> {
        if !self.is_tag_key_allowed(key) {
            return Err(ContentDiagnostic::NotAllowedTagKey { key: key.clone() });
        }

        if let Some(definition) = self.tag_definition_for_key(key) {
            definition.normalize_value(key, value)
        } else {
            // If the key is allowed but not defined, we parse as a text tag or key only tag
            // with no constraints as fallback
            let fallback_definition = if value.is_empty() {
                TagDefinition::KeyOnly
            } else {
                TagDefinition::Text(TextTagValueDefinition {
                    min_length: None,
                    max_length: None,
                    allowed_values: None,
                })
            };

            fallback_definition
                .normalize_value(key, value)
                // This fallback definition is only for simplicyty, so we must strip it from diagnostic if the value is unparseable.
                .map_err(|diagnostic| diagnostic.strip_definition_for_unparseable_tag_value())
        }
    }

    /// Same as `normalize_tag` but for a set of values.
    fn normalize_set_tag(
        &self,
        key: &TagKey,
        values: &HashSet<String>,
    ) -> Result<Tag, ContentDiagnostic> {
        if !self.is_tag_key_allowed(key) {
            return Err(ContentDiagnostic::NotAllowedTagKey { key: key.clone() });
        }

        if let Some(definition) = self.tag_definition_for_key(key) {
            definition.normalize_set_value(key, values)
        } else {
            // If the key is allowed but not defined, we parse as a text tag or key only tag
            // with no constraints as fallback
            let fallback_definition = TagDefinition::TextSet(TextTagValueDefinition {
                min_length: None,
                max_length: None,
                allowed_values: None,
            });

            fallback_definition
                .normalize_set_value(key, values)
                // Same with `normalize_tag`
                .map_err(|diagnostic| diagnostic.strip_definition_for_unparseable_tag_value())
        }
    }

    /// Diagnoses a set of tags against the schema, returning any diagnostics found.
    /// This method assumes that each of the tags has already been normalized and validated against the schema.
    /// So, this method is mainly for checking combination of tags is conformant.
    fn diagnose_tags_combination(&self, tags: &[Tag]) -> HashSet<ContentDiagnostic> {
        let mut diagnostics = HashSet::new();

        // 1. Check if any required tags are missing.
        let keys = tags.iter().map(|tag| tag.key()).collect::<HashSet<_>>();

        if let Some(required) = &self.required {
            required
                .iter()
                .filter(|(key, _)| !keys.contains(key))
                .map(|(key, definition)| ContentDiagnostic::MissingRequiredTag {
                    key: key.clone(),
                    definition: definition.clone(),
                })
                .for_each(|diagnostic| {
                    diagnostics.insert(diagnostic);
                });
        }

        diagnostics
    }

    /// Constructs a search term based on the schema's rules for a given tag key and candidate values.
    fn construct_search_term(&self, key: &TagKey, candidates: &[String]) -> Option<Term> {
        if !self.is_tag_key_allowed(key) {
            return None;
        }

        if let Some(definition) = self.tag_definition_for_key(key) {
            match definition {
                TagDefinition::Text(definition) | TagDefinition::TextSet(definition) => candidates
                    .iter()
                    .map(|candidate| definition.parse(candidate))
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
                    .map(|values| TagPredicate::new_text_match(values.as_slice()))
                    .and_then(|predicate| predicate.map(|p| Term::new_tag(key.clone(), p))),
                TagDefinition::Integer(definition) | TagDefinition::IntegerSet(definition) => {
                    candidates
                        .iter()
                        .map(|candidate| definition.parse(candidate))
                        .collect::<Result<Vec<_>, _>>()
                        .ok()
                        .map(|values| TagPredicate::new_integer_match(values.as_slice()))
                        .and_then(|predicate| predicate.map(|p| Term::new_tag(key.clone(), p)))
                }
                TagDefinition::Real(definition) | TagDefinition::RealSet(definition) => candidates
                    .iter()
                    .map(|candidate| definition.parse(candidate))
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
                    .map(|values| TagPredicate::new_real_match(values.as_slice()))
                    .and_then(|predicate| predicate.map(|p| Term::new_tag(key.clone(), p))),
                TagDefinition::KeyOnly => None,
            }
        } else {
            let fallback_definition = TextTagValueDefinition {
                min_length: None,
                max_length: None,
                allowed_values: None,
            };

            candidates
                .iter()
                .map(|candidate| fallback_definition.parse(candidate))
                .collect::<Result<Vec<_>, _>>()
                .ok()
                .map(|values| TagPredicate::new_text_match(values.as_slice()))
                .and_then(|predicate| predicate.map(|p| Term::new_tag(key.clone(), p)))
        }
    }
}

impl Default for TagSchemaV0 {
    fn default() -> Self {
        Self {
            allow_additional_tags: true,
            required: None,
            optional: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(tag = "type", rename_all = "snake_case")]
/// Represents the definition of a tag
pub(crate) enum TagDefinition {
    KeyOnly,
    Text(TextTagValueDefinition),
    Integer(IntegerTagValueDefinition),
    Real(RealTagValueDefinition),
    TextSet(TextTagValueDefinition),
    IntegerSet(IntegerTagValueDefinition),
    RealSet(RealTagValueDefinition),
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

    /// Normalizes a single value into a tag based on the definition's rules.
    fn normalize_value(&self, key: &TagKey, value: &str) -> Result<Tag, ContentDiagnostic> {
        match self {
            TagDefinition::KeyOnly => {
                if value.is_empty() {
                    Ok(Tag::KeyOnly { key: key.clone() })
                } else {
                    Err(ContentDiagnostic::UnparseableTagValue {
                        key: key.clone(),
                        definition: Some(self.clone()),
                    })
                }
            }
            TagDefinition::Text(def) => def
                .parse(value)
                .map_err(|error| error.with_definition(key, self))
                .map(|v| Tag::Text {
                    key: key.clone(),
                    value: v,
                }),
            TagDefinition::Integer(def) => def
                .parse(value)
                .map_err(|error| error.with_definition(key, self))
                .map(|v| Tag::Integer {
                    key: key.clone(),
                    value: v,
                }),
            TagDefinition::Real(def) => def
                .parse(value)
                .map_err(|error| error.with_definition(key, self))
                .map(|v| Tag::Real {
                    key: key.clone(),
                    value: v,
                }),
            TagDefinition::TextSet(_)
            | TagDefinition::IntegerSet(_)
            | TagDefinition::RealSet(_) => Err(ContentDiagnostic::UnparseableTagValue {
                key: key.clone(),
                definition: Some(self.clone()),
            }),
        }
    }

    /// Noarmalizes a set of values into a tag based on the definition's rules.
    fn normalize_set_value(
        &self,
        key: &TagKey,
        values: &HashSet<String>,
    ) -> Result<Tag, ContentDiagnostic> {
        match self {
            TagDefinition::KeyOnly
            | TagDefinition::Text(_)
            | TagDefinition::Integer(_)
            | TagDefinition::Real(_) => Err(ContentDiagnostic::UnparseableTagValue {
                key: key.clone(),
                definition: Some(self.clone()),
            }),
            TagDefinition::TextSet(def) => values
                .iter()
                .map(|value| def.parse(value))
                // For text set tag, we don't need to check for duplicates because arguments are already in a HashSet
                .collect::<Result<HashSet<_>, _>>()
                .map_err(|error| error.with_definition(key, self))
                .map(|v| Tag::TextSet {
                    key: key.clone(),
                    values: v,
                }),
            TagDefinition::IntegerSet(def) => values
                .iter()
                .map(|value| def.parse(value))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.with_definition(key, self))
                // We need to chedk for duplicates after parsing because the input is a HashSet of strings, but the parsed values might not be unique
                .and_then(|values| {
                    let mut values_set = HashSet::new();

                    if values.iter().any(|value| !values_set.insert(value.clone())) {
                        Err(ContentDiagnostic::DuplicateSetValue { key: key.clone() })
                    } else {
                        Ok(values_set)
                    }
                })
                .map(|v| Tag::IntegerSet {
                    key: key.clone(),
                    values: v,
                }),
            TagDefinition::RealSet(def) => values
                .iter()
                .map(|value| def.parse(value))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.with_definition(key, self))
                // We need to chedk for duplicates after parsing because the input is a HashSet of strings, but the parsed values might not be unique
                .and_then(|values| {
                    let mut values_set = HashSet::new();

                    if values.iter().any(|value| !values_set.insert(value.clone())) {
                        Err(ContentDiagnostic::DuplicateSetValue { key: key.clone() })
                    } else {
                        Ok(values_set)
                    }
                })
                .map(|values| Tag::RealSet {
                    key: key.clone(),
                    values,
                }),
        }
    }
}

/// A trait for generic tag value definitions.
trait TagValueDefinition<T> {
    /// Validates the format of the tag value definition itself.
    fn validate_format(&self) -> Result<(), String>;
    /// Parses a value that conforms to this definition.
    fn parse(&self, value: &str) -> Result<T, TagValueDefinitionViolation>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Represents violations that can occur when parsing tag values.
enum TagValueDefinitionViolation {
    /// The value could not be parsed into the expected type.
    Unparseable,
    /// The value is not allowed based on the definition's constraints.
    NotAllowedValue,
}

impl TagValueDefinitionViolation {
    fn with_definition(self, key: &TagKey, definition: &TagDefinition) -> ContentDiagnostic {
        match self {
            TagValueDefinitionViolation::Unparseable => ContentDiagnostic::UnparseableTagValue {
                key: key.clone(),
                definition: Some(definition.clone()),
            },
            TagValueDefinitionViolation::NotAllowedValue => ContentDiagnostic::NotAllowedTagValue {
                key: key.clone(),
                definition: definition.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct TextTagValueDefinition {
    /// Minimum/maximum length of the text value in Unicode code points.
    /// If not specified, the TextTagValue limit still applies.
    /// Exclusive with `allowed_values`.
    min_length: Option<NonZeroU8>,
    max_length: Option<NonZeroU8>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min_length`/`max_length`.
    allowed_values: Option<BTreeSet<TextTagValue>>,
}

impl TextTagValueDefinition {
    pub(crate) fn min_length(&self) -> Option<NonZeroU8> {
        self.min_length
    }

    pub(crate) fn max_length(&self) -> Option<NonZeroU8> {
        self.max_length
    }

    pub(crate) fn allowed_values(&self) -> Option<&BTreeSet<TextTagValue>> {
        self.allowed_values.as_ref()
    }
}

impl TagValueDefinition<TextTagValue> for TextTagValueDefinition {
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

    fn parse(&self, value: &str) -> Result<TextTagValue, TagValueDefinitionViolation> {
        // 1. Check if the value can be converted to a `TextTagValue`.
        let value = if let Some(value) = TextTagValue::new(value) {
            value
        } else {
            return Err(TagValueDefinitionViolation::Unparseable);
        };

        // 2. Check if the value is in the allowed values set, if specified.
        if let Some(allowed_values) = &self.allowed_values
            && !allowed_values.contains(&value)
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        // 3. Check the length constraints.
        let length = value.as_ref().chars().count();
        if let Some(min_length) = self.min_length
            && length < usize::from(min_length.get())
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }
        if let Some(max_length) = self.max_length
            && length > usize::from(max_length.get())
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        Ok(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct IntegerTagValueDefinition {
    /// Minimum/maximum value, if not specified, there is no limitation.
    /// Exclusive with `allowed_values`.
    min: Option<IntegerTagValue>,
    max: Option<IntegerTagValue>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min`/`max`.
    allowed_values: Option<BTreeSet<IntegerTagValue>>,
}

impl IntegerTagValueDefinition {
    pub(crate) fn min_value(&self) -> Option<&IntegerTagValue> {
        self.min.as_ref()
    }

    pub(crate) fn max_value(&self) -> Option<&IntegerTagValue> {
        self.max.as_ref()
    }

    pub(crate) fn allowed_values(&self) -> Option<&BTreeSet<IntegerTagValue>> {
        self.allowed_values.as_ref()
    }
}

impl TagValueDefinition<IntegerTagValue> for IntegerTagValueDefinition {
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

    fn parse(&self, value: &str) -> Result<IntegerTagValue, TagValueDefinitionViolation> {
        // 1. Check if the value can be converted to an `IntegerTagValue`.
        let value = if let Ok(value) = value.parse::<i64>()
            && let Some(value) = IntegerTagValue::new(value)
        {
            value
        } else {
            return Err(TagValueDefinitionViolation::Unparseable);
        };

        // 2. Check if the value is in the allowed values set, if specified.
        if let Some(allowed_values) = &self.allowed_values
            && !allowed_values.contains(&value)
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        // 3. Check the min/max constraints.
        if let Some(min) = &self.min
            && value < *min
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }
        if let Some(max) = &self.max
            && value > *max
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        Ok(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct RealTagValueDefinition {
    /// Minimum/maximum value, if not specified, there is no limitation.
    /// Exclusive with `allowed_values`.
    min: Option<RealTagValue>,
    max: Option<RealTagValue>,
    /// A set of allowed values. If specified, the value must be one of these.
    /// Exclusive with `min`/`max`.
    allowed_values: Option<BTreeSet<RealTagValue>>,
}

impl RealTagValueDefinition {
    pub(crate) fn min_value(&self) -> Option<&RealTagValue> {
        self.min.as_ref()
    }

    pub(crate) fn max_value(&self) -> Option<&RealTagValue> {
        self.max.as_ref()
    }

    pub(crate) fn allowed_values(&self) -> Option<&BTreeSet<RealTagValue>> {
        self.allowed_values.as_ref()
    }
}

impl TagValueDefinition<RealTagValue> for RealTagValueDefinition {
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

    fn parse(&self, value: &str) -> Result<RealTagValue, TagValueDefinitionViolation> {
        // 1. Check if the value can be converted to a `RealTagValue`.
        let value = if let Ok(value) = value.parse::<f64>()
            && let Some(value) = RealTagValue::new(value)
        {
            value
        } else {
            return Err(TagValueDefinitionViolation::Unparseable);
        };

        // 2. Check if the value is in the allowed values set, if specified.
        if let Some(allowed_values) = &self.allowed_values
            && !allowed_values.contains(&value)
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        // 3. Check the min/max constraints.
        if let Some(min) = &self.min
            && value < *min
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }
        if let Some(max) = &self.max
            && value > *max
        {
            return Err(TagValueDefinitionViolation::NotAllowedValue);
        }

        Ok(value)
    }
}

#[cfg(test)]
mod tests;

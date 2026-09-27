use galerie_api::models as api;
use galerie_api::models::{
    Content, ContentDiagnostic, IntegerSetTag, IntegerTag, IntegerTagValue, KeyOnlyTag, MediaType,
    RealSetTag, RealTag, RealTagValue, Tag, TextSetTag, TextTag, TextTagValue,
};

use crate::domain;

impl From<domain::Content> for Content {
    fn from(content: domain::Content) -> Self {
        Content {
            id: content.id().as_ref().to_string(),
            media_type: content.media_type().into(),
            content_url: content.content_url().to_string(),
            thumbnail_url: content.thumbnail_url().to_string(),
            tags: content
                .tags()
                .as_ref()
                .values()
                .map(|tag| tag.clone().into())
                .collect(),
            diagnostics: content
                .diagnostics()
                .iter()
                .map(|diag| diag.clone().into())
                .collect(),
        }
    }
}

impl From<domain::MediaType> for MediaType {
    fn from(media_type: domain::MediaType) -> Self {
        match media_type {
            domain::MediaType::Avif => MediaType::ImageSlashAvif,
        }
    }
}

impl From<MediaType> for domain::MediaType {
    fn from(media_type: MediaType) -> Self {
        match media_type {
            MediaType::ImageSlashAvif => domain::MediaType::Avif,
        }
    }
}

impl From<domain::tag::Tag> for Tag {
    fn from(tag: domain::tag::Tag) -> Self {
        match tag {
            domain::tag::Tag::KeyOnly { key } => Self::KeyOnlyTag(KeyOnlyTag::new(
                key.as_ref().to_string(),
                api::KeyOnlyTagType::KeyOnly,
            )),
            domain::tag::Tag::Text { key, value } => Self::TextTag(TextTag::new(
                key.as_ref().to_string(),
                api::TextTagType::Text,
                value.as_ref().to_string(),
            )),
            domain::tag::Tag::Integer { key, value } => Self::IntegerTag(IntegerTag::new(
                key.as_ref().to_string(),
                api::IntegerTagType::Integer,
                *value.as_ref(),
            )),
            domain::tag::Tag::Real { key, value } => Self::RealTag(RealTag::new(
                key.as_ref().to_string(),
                api::RealTagType::Real,
                *value.as_ref(),
            )),
            domain::tag::Tag::TextSet { key, values } => Self::TextSetTag(TextSetTag::new(
                key.as_ref().to_string(),
                api::TextSetTagType::TextSet,
                values
                    .into_iter()
                    .map(|v| TextTagValue(v.as_ref().to_string()))
                    .collect(),
            )),
            domain::tag::Tag::IntegerSet { key, values } => {
                Self::IntegerSetTag(IntegerSetTag::new(
                    key.as_ref().to_string(),
                    api::IntegerSetTagType::IntegerSet,
                    values
                        .into_iter()
                        .map(|v| IntegerTagValue(*v.as_ref()))
                        .collect(),
                ))
            }
            domain::tag::Tag::RealSet { key, values } => Self::RealSetTag(RealSetTag::new(
                key.as_ref().to_string(),
                api::RealSetTagType::RealSet,
                values
                    .into_iter()
                    .map(|v| RealTagValue(*v.as_ref()))
                    .collect(),
            )),
        }
    }
}

impl From<domain::ContentDiagnostic> for ContentDiagnostic {
    fn from(value: domain::ContentDiagnostic) -> Self {
        match value {
            domain::ContentDiagnostic::InvalidKey { key } => Self::InvalidKeyDiagnostic(
                api::InvalidKeyDiagnostic::new(key, api::InvalidKeyDiagnosticKind::InvalidKey),
            ),
            domain::ContentDiagnostic::DuplicateKey { key } => {
                Self::DuplicateKeyDiagnostic(api::DuplicateKeyDiagnostic::new(
                    key.as_ref().to_string(),
                    api::DuplicateKeyDiagnosticKind::DuplicateKey,
                ))
            }
            domain::ContentDiagnostic::UnsupportedXmpValueType { key } => {
                Self::UnsupportedXmpValueTypeDiagnostic(
                    api::UnsupportedXmpValueTypeDiagnostic::new(
                        key.as_ref().to_string(),
                        api::UnsupportedXmpValueTypeDiagnosticKind::UnsupportedXmpValueType,
                    ),
                )
            }
            domain::ContentDiagnostic::NotAllowedTagKey { key } => {
                Self::NotAllowedTagKeyDiagnostic(api::NotAllowedTagKeyDiagnostic::new(
                    key.as_ref().to_string(),
                    api::NotAllowedTagKeyDiagnosticKind::NotAllowedTagKey,
                ))
            }
            domain::ContentDiagnostic::UnparseableTagValue { key, definition } => {
                let definition = definition.map(|definition| tag_definition(&key, definition));
                let mut diagnostic = api::UnparseableTagValueDiagnostic::new(
                    key.as_ref().to_string(),
                    api::UnparseableTagValueDiagnosticKind::UnparseableTagValue,
                );
                diagnostic.definition = definition;
                Self::UnparseableTagValueDiagnostic(diagnostic)
            }
            domain::ContentDiagnostic::DuplicateSetValue { key } => {
                Self::DuplicateSetValueDiagnostic(api::DuplicateSetValueDiagnostic::new(
                    key.as_ref().to_string(),
                    api::DuplicateSetValueDiagnosticKind::DuplicateSetValue,
                ))
            }
            domain::ContentDiagnostic::NotAllowedTagValue { key, definition } => {
                let definition = tag_definition(&key, definition);
                Self::NotAllowedTagValueDiagnostic(api::NotAllowedTagValueDiagnostic::new(
                    key.as_ref().to_string(),
                    api::NotAllowedTagValueDiagnosticKind::NotAllowedTagValue,
                    definition,
                ))
            }
            domain::ContentDiagnostic::MissingRequiredTag { key, definition } => {
                let definition = tag_definition(&key, definition);
                Self::MissingRequiredTagDiagnostic(api::MissingRequiredTagDiagnostic::new(
                    key.as_ref().to_string(),
                    api::MissingRequiredTagDiagnosticKind::MissingRequiredTag,
                    definition,
                ))
            }
        }
    }
}

fn tag_definition(
    key: &domain::tag::TagKey,
    definition: domain::tag_schema::TagDefinition,
) -> api::TagDefinition {
    use domain::tag_schema::TagDefinition as DomainDefinition;

    let key = key.as_ref().to_string();
    match definition {
        DomainDefinition::KeyOnly => api::TagDefinition::KeyOnlyTagDefinition(
            api::KeyOnlyTagDefinition::new(key, api::KeyOnlyTagType::KeyOnly),
        ),
        DomainDefinition::Text(definition) => text_definition(key, definition, false),
        DomainDefinition::TextSet(definition) => text_definition(key, definition, true),
        DomainDefinition::Integer(definition) => integer_definition(key, definition, false),
        DomainDefinition::IntegerSet(definition) => integer_definition(key, definition, true),
        DomainDefinition::Real(definition) => real_definition(key, definition, false),
        DomainDefinition::RealSet(definition) => real_definition(key, definition, true),
    }
}

fn text_definition(
    key: String,
    definition: domain::tag_schema::TextTagValueDefinition,
    is_set: bool,
) -> api::TagDefinition {
    let min_length = definition
        .min_length()
        .map(|length| u32::from(length.get()));
    let max_length = definition
        .max_length()
        .map(|length| u32::from(length.get()));
    let allowed_values = definition.allowed_values().map(|values| {
        values
            .iter()
            .map(|value| TextTagValue(value.as_ref().to_string()))
            .collect()
    });

    if is_set {
        let mut result = api::TextSetTagDefinition::new(key, api::TextSetTagType::TextSet);
        result.min_length = min_length;
        result.max_length = max_length;
        result.allowed_values = allowed_values;
        api::TagDefinition::TextSetTagDefinition(result)
    } else {
        let mut result = api::TextTagDefinition::new(key, api::TextTagType::Text);
        result.min_length = min_length;
        result.max_length = max_length;
        result.allowed_values = allowed_values;
        api::TagDefinition::TextTagDefinition(result)
    }
}

fn integer_definition(
    key: String,
    definition: domain::tag_schema::IntegerTagValueDefinition,
    is_set: bool,
) -> api::TagDefinition {
    let min = definition.min_value().map(|value| *value.as_ref());
    let max = definition.max_value().map(|value| *value.as_ref());
    let allowed_values = definition.allowed_values().map(|values| {
        values
            .iter()
            .map(|value| IntegerTagValue(*value.as_ref()))
            .collect()
    });

    if is_set {
        let mut result = api::IntegerSetTagDefinition::new(key, api::IntegerSetTagType::IntegerSet);
        result.min = min;
        result.max = max;
        result.allowed_values = allowed_values;
        api::TagDefinition::IntegerSetTagDefinition(result)
    } else {
        let mut result = api::IntegerTagDefinition::new(key, api::IntegerTagType::Integer);
        result.min = min;
        result.max = max;
        result.allowed_values = allowed_values;
        api::TagDefinition::IntegerTagDefinition(result)
    }
}

fn real_definition(
    key: String,
    definition: domain::tag_schema::RealTagValueDefinition,
    is_set: bool,
) -> api::TagDefinition {
    let min = definition.min_value().map(|value| *value.as_ref());
    let max = definition.max_value().map(|value| *value.as_ref());
    let allowed_values = definition.allowed_values().map(|values| {
        values
            .iter()
            .map(|value| RealTagValue(*value.as_ref()))
            .collect()
    });

    if is_set {
        let mut result = api::RealSetTagDefinition::new(key, api::RealSetTagType::RealSet);
        result.min = min;
        result.max = max;
        result.allowed_values = allowed_values;
        api::TagDefinition::RealSetTagDefinition(result)
    } else {
        let mut result = api::RealTagDefinition::new(key, api::RealTagType::Real);
        result.min = min;
        result.max = max;
        result.allowed_values = allowed_values;
        api::TagDefinition::RealTagDefinition(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminator_values_follow_the_api_contract() {
        let key = domain::tag::TagKey::new("category").unwrap();
        let tag: Tag = domain::tag::Tag::KeyOnly { key: key.clone() }.into();
        let tag_json = serde_json::to_value(tag).unwrap();
        assert_eq!(tag_json["type"], "keyOnly");
        let _: Tag = serde_json::from_value(tag_json).unwrap();

        let diagnostic: ContentDiagnostic = domain::ContentDiagnostic::MissingRequiredTag {
            key,
            definition: domain::tag_schema::TagDefinition::KeyOnly,
        }
        .into();
        let diagnostic_json = serde_json::to_value(diagnostic).unwrap();
        assert_eq!(diagnostic_json["kind"], "missingRequiredTag");
        assert_eq!(diagnostic_json["definition"]["type"], "keyOnly");
        let _: ContentDiagnostic = serde_json::from_value(diagnostic_json).unwrap();
    }
}

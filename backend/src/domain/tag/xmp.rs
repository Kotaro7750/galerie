use crate::domain::{tag::TagSet, tag_schema::TagSchema};

pub(crate) mod parse;
pub(crate) mod serialize;

const TAG_NAMESPACE_URI: &str = "galerie";

/// Check if the given tags can be serialized to XMP and then parsed back to the same tags without any diagnostics.
pub(crate) fn check_xmp_roundtrip(tags: &TagSet, tag_schema: &TagSchema) -> bool {
    let Ok(xmp) = serialize::create_xmp_with_tags(tags) else {
        return false;
    };
    matches!(
        parse::parse_metadata(&xmp, tag_schema),
        Ok((parsed, diagnostics)) if diagnostics.is_empty() && parsed == *tags
    )
}

#[cfg(test)]
mod tests {
    use crate::domain::tag::{RealTagValue, Tag, TagKey, TagSet};
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn xmp_serialization_round_trips_typed_tags_and_escaped_text() {
        let schema: crate::domain::tag_schema::TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\noptional:\n  title: { type: text }\n  rating: { type: integer }\n  score: { type: real }\n  labels: { type: text_set }\n  ranks: { type: integer_set }\n  weights: { type: real_set }\n  flag: { type: key_only }\n"
        ).unwrap();
        let tags = [
            schema
                .normalize_tag(TagKey::new("title").unwrap(), "A & B <C>".to_string())
                .unwrap(),
            schema
                .normalize_tag(TagKey::new("rating").unwrap(), "5".to_string())
                .unwrap(),
            schema
                .normalize_tag(TagKey::new("score").unwrap(), "1.5".to_string())
                .unwrap(),
            schema
                .normalize_set_tag(
                    TagKey::new("labels").unwrap(),
                    &HashSet::from(["a & b".to_string()]),
                )
                .unwrap(),
            schema
                .normalize_set_tag(
                    TagKey::new("ranks").unwrap(),
                    &HashSet::from(["1".to_string(), "2".to_string()]),
                )
                .unwrap(),
            schema
                .normalize_set_tag(
                    TagKey::new("weights").unwrap(),
                    &HashSet::from(["1.5".to_string(), "2.5".to_string()]),
                )
                .unwrap(),
            schema
                .normalize_tag(TagKey::new("flag").unwrap(), String::new())
                .unwrap(),
        ];
        let tags = TagSet::new(&tags).unwrap();

        let (parsed, diagnostics) = parse::parse_metadata(
            serialize::create_xmp_with_tags(&tags).unwrap().as_ref(),
            &schema,
        )
        .unwrap();

        assert!(diagnostics.is_empty(), "{diagnostics:?}");
        assert_eq!(parsed, tags);
    }

    #[test]
    fn xmp_serialization_round_trips_finite_real_extremes() {
        let schema: crate::domain::tag_schema::TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\noptional:\n  number: { type: real }\n",
        )
        .unwrap();
        for value in [
            f64::MAX,
            f64::MIN,
            f64::MIN_POSITIVE,
            f64::from_bits(1),
            -0.0,
        ] {
            let tag = Tag::Real {
                key: TagKey::new("number").unwrap(),
                value: RealTagValue::new(value).unwrap(),
            };
            let tags = TagSet::new(&[tag]).unwrap();

            let (parsed, diagnostics) = parse::parse_metadata(
                serialize::create_xmp_with_tags(&tags).unwrap().as_ref(),
                &schema,
            )
            .unwrap();

            assert!(diagnostics.is_empty(), "{value}: {diagnostics:?}");
            assert_eq!(parsed, tags, "{value}");
        }
    }

    #[test]
    fn xmp_serialization_round_trips_empty_sets() {
        let schema: crate::domain::tag_schema::TagSchema = serde_yaml::from_str
(
            "version: '0'\nallow_additional_tags: false\noptional:\n  labels: { type: text_set }\n  ranks: { type: integer_set }\n  weights: { type: real_set }\n",
        )
        .unwrap();
        for (_, tag) in [
            (
                TagKey::new("labels").unwrap(),
                Tag::TextSet {
                    key: TagKey::new("labels").unwrap(),
                    values: HashSet::new(),
                },
            ),
            (
                TagKey::new("ranks").unwrap(),
                Tag::IntegerSet {
                    key: TagKey::new("ranks").unwrap(),
                    values: HashSet::new(),
                },
            ),
            (
                TagKey::new("weights").unwrap(),
                Tag::RealSet {
                    key: TagKey::new("weights").unwrap(),
                    values: HashSet::new(),
                },
            ),
        ] {
            let tags = TagSet::new(&[tag]).unwrap();

            let (parsed, diagnostics) = parse::parse_metadata(
                serialize::create_xmp_with_tags(&tags).unwrap().as_ref(),
                &schema,
            )
            .unwrap();

            assert!(diagnostics.is_empty(), "{diagnostics:?}");
            assert_eq!(parsed, tags);
        }
    }
}

use super::*;
use unicode_normalization::UnicodeNormalization;

// === Test helpers ===

fn key(value: &str) -> TagKey {
    TagKey::new(value).unwrap()
}

fn schema(
    allow_additional_tags: bool,
    required: Vec<(&str, TagDefinition)>,
    optional: Vec<(&str, TagDefinition)>,
) -> TagSchema {
    TagSchema::V0(TagSchemaV0 {
        allow_additional_tags,
        required: Some(
            required
                .into_iter()
                .map(|(name, def)| (key(name), def))
                .collect(),
        ),
        optional: Some(
            optional
                .into_iter()
                .map(|(name, def)| (key(name), def))
                .collect(),
        ),
    })
}

fn text_definition() -> TextTagValueDefinition {
    TextTagValueDefinition {
        min_length: None,
        max_length: None,
        allowed_values: None,
    }
}

fn text_range(min_length: u8, max_length: u8) -> TextTagValueDefinition {
    TextTagValueDefinition {
        min_length: Some(NonZeroU8::new(min_length).unwrap()),
        max_length: Some(NonZeroU8::new(max_length).unwrap()),
        allowed_values: None,
    }
}

fn integer_definition() -> IntegerTagValueDefinition {
    IntegerTagValueDefinition {
        min: None,
        max: None,
        allowed_values: None,
    }
}

fn integer_range(min: i64, max: i64) -> IntegerTagValueDefinition {
    IntegerTagValueDefinition {
        min: Some(integer(min)),
        max: Some(integer(max)),
        allowed_values: None,
    }
}

fn real_definition() -> RealTagValueDefinition {
    RealTagValueDefinition {
        min: None,
        max: None,
        allowed_values: None,
    }
}

fn real_range(min: f64, max: f64) -> RealTagValueDefinition {
    RealTagValueDefinition {
        min: Some(real(min)),
        max: Some(real(max)),
        allowed_values: None,
    }
}

fn values(items: &[&str]) -> HashSet<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

fn integer(value: i64) -> IntegerTagValue {
    IntegerTagValue::new(value).unwrap()
}

fn real(value: f64) -> RealTagValue {
    RealTagValue::new(value).unwrap()
}

fn search_values(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

// === TagSchema ===

#[test]
fn v0_schema_validate_format_accepts_empty_definitions() {
    assert!(schema(false, vec![], vec![]).validate_format().is_ok());
}

#[test]
fn v0_schema_validate_format_accepts_distinct_required_and_optional_tags() {
    assert!(
        schema(
            false,
            vec![("required", TagDefinition::KeyOnly)],
            vec![("optional", TagDefinition::Text(text_definition()))],
        )
        .validate_format()
        .is_ok()
    );
}

#[test]
fn v0_schema_validate_format_rejects_key_in_required_and_optional() {
    assert!(
        schema(
            false,
            vec![("same", TagDefinition::KeyOnly)],
            vec![("same", TagDefinition::KeyOnly)],
        )
        .validate_format()
        .is_err()
    );
}

#[test]
fn v0_schema_validate_format_rejects_invalid_required_definition() {
    let invalid = TagDefinition::Text(TextTagValueDefinition {
        min_length: NonZeroU8::new(2),
        max_length: NonZeroU8::new(1),
        allowed_values: None,
    });
    assert!(
        schema(false, vec![("required", invalid)], vec![])
            .validate_format()
            .is_err()
    );
}

#[test]
fn v0_schema_validate_format_rejects_invalid_optional_definition() {
    let invalid = TagDefinition::Text(TextTagValueDefinition {
        min_length: NonZeroU8::new(2),
        max_length: NonZeroU8::new(1),
        allowed_values: None,
    });
    assert!(
        schema(false, vec![], vec![("optional", invalid)])
            .validate_format()
            .is_err()
    );
}

#[test]
fn v0_schema_diagnose_tags_combination_reports_missing_required_tag_for_empty_tags() {
    let definition = TagDefinition::Text(text_definition());
    let schema = schema(
        false,
        vec![("required", definition.clone())],
        vec![("optional", TagDefinition::KeyOnly)],
    );
    let missing = ContentDiagnostic::MissingRequiredTag {
        key: key("required"),
        definition,
    };

    assert_eq!(schema.diagnose_tags_combination(&[]), [missing].into());
}

#[test]
fn v0_schema_diagnose_tags_combination_reports_missing_required_tag_with_only_optional_tag() {
    let definition = TagDefinition::Text(text_definition());
    let schema = schema(
        false,
        vec![("required", definition.clone())],
        vec![("optional", TagDefinition::KeyOnly)],
    );
    let missing = ContentDiagnostic::MissingRequiredTag {
        key: key("required"),
        definition,
    };

    assert_eq!(
        schema.diagnose_tags_combination(&[Tag::KeyOnly {
            key: key("optional"),
        }]),
        [missing].into()
    );
}

#[test]
fn v0_schema_diagnose_tags_combination_accepts_present_required_tag() {
    let schema = schema(
        false,
        vec![("required", TagDefinition::Text(text_definition()))],
        vec![("optional", TagDefinition::KeyOnly)],
    );

    assert!(
        schema
            .diagnose_tags_combination(&[Tag::Text {
                key: key("required"),
                value: TextTagValue::new("present").unwrap(),
            }])
            .is_empty()
    );
}

#[test]
fn v0_schema_normalize_tag_rejects_set_definitions() {
    let set_definitions = [
        TagDefinition::TextSet(text_definition()),
        TagDefinition::IntegerSet(integer_definition()),
        TagDefinition::RealSet(real_definition()),
    ];
    for definition in set_definitions {
        let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
        let tag_key = key("tag");
        assert_eq!(
            strict_schema.normalize_tag(tag_key.clone(), "1".to_string()),
            Err(ContentDiagnostic::UnparseableTagValue {
                key: tag_key,
                definition: Some(definition),
            })
        );
    }
}

#[test]
fn v0_schema_normalize_tag_rejects_undefined_key_when_additional_tags_disabled() {
    let strict_schema = schema(false, vec![], vec![]);
    assert_eq!(
        strict_schema.normalize_tag(key("unknown"), "value".to_string()),
        Err(ContentDiagnostic::NotAllowedTagKey {
            key: key("unknown"),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_parses_key_only() {
    let strict_schema = schema(false, vec![], vec![("tag", TagDefinition::KeyOnly)]);
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), String::new()),
        Ok(Tag::KeyOnly { key: key("tag") })
    );
}

#[test]
fn v0_schema_normalize_tag_rejects_value_for_key_only_definition() {
    let strict_schema = schema(false, vec![], vec![("tag", TagDefinition::KeyOnly)]);
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "value".to_string()),
        Err(ContentDiagnostic::UnparseableTagValue {
            key: key("tag"),
            definition: Some(TagDefinition::KeyOnly),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_reports_unparseable_value_with_definition() {
    let definition = TagDefinition::Integer(integer_definition());
    let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "text".to_string()),
        Err(ContentDiagnostic::UnparseableTagValue {
            key: key("tag"),
            definition: Some(definition),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_reports_disallowed_value_with_definition() {
    let definition = TagDefinition::Integer(integer_range(1, 2));
    let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "3".to_string()),
        Err(ContentDiagnostic::NotAllowedTagValue {
            key: key("tag"),
            definition,
        })
    );
}

#[test]
fn v0_schema_normalize_tag_parses_text() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::Text(text_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "text".to_string()),
        Ok(Tag::Text {
            key: key("tag"),
            value: TextTagValue::new("text").unwrap(),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_parses_integer() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::Integer(integer_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "42".to_string()),
        Ok(Tag::Integer {
            key: key("tag"),
            value: integer(42),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_parses_real() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::Real(real_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_tag(key("tag"), "1.5".to_string()),
        Ok(Tag::Real {
            key: key("tag"),
            value: real(1.5),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_defaults_undefined_value_to_text() {
    let additional = schema(true, vec![], vec![]);
    assert_eq!(
        additional.normalize_tag(key("extra"), "1.5".to_string()),
        Ok(Tag::Text {
            key: key("extra"),
            value: TextTagValue::new("1.5").unwrap(),
        })
    );
}

#[test]
fn v0_schema_normalize_tag_defaults_undefined_empty_value_to_key_only() {
    let additional = schema(true, vec![], vec![]);
    assert_eq!(
        additional.normalize_tag(key("empty"), String::new()),
        Ok(Tag::KeyOnly { key: key("empty") })
    );
}

#[test]
fn v0_schema_normalize_tag_omits_definition_for_invalid_undefined_value() {
    let additional = schema(true, vec![], vec![]);
    assert_eq!(
        additional.normalize_tag(key("extra"), "a".repeat(256)),
        Err(ContentDiagnostic::UnparseableTagValue {
            key: key("extra"),
            definition: None,
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_rejects_simple_definitions() {
    let simple_definitions = [
        TagDefinition::KeyOnly,
        TagDefinition::Text(text_definition()),
        TagDefinition::Integer(integer_definition()),
        TagDefinition::Real(real_definition()),
    ];
    for definition in simple_definitions {
        let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
        let tag_key = key("tag");
        assert_eq!(
            strict_schema.normalize_set_tag(tag_key.clone(), &values(&["1"])),
            Err(ContentDiagnostic::UnparseableTagValue {
                key: tag_key,
                definition: Some(definition),
            })
        );
    }
}

#[test]
fn v0_schema_normalize_set_tag_rejects_undefined_key_when_additional_tags_disabled() {
    let strict_schema = schema(false, vec![], vec![]);
    assert_eq!(
        strict_schema.normalize_set_tag(key("unknown"), &values(&["value"])),
        Err(ContentDiagnostic::NotAllowedTagKey {
            key: key("unknown"),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_parses_text_set() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::TextSet(text_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["a", "b"])),
        Ok(Tag::TextSet {
            key: key("tag"),
            values: [
                TextTagValue::new("a").unwrap(),
                TextTagValue::new("b").unwrap()
            ]
            .into(),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_parses_integer_set() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::IntegerSet(integer_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["1", "2"])),
        Ok(Tag::IntegerSet {
            key: key("tag"),
            values: [integer(1), integer(2)].into(),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_parses_real_set() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::RealSet(real_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["1.5", "2.5"])),
        Ok(Tag::RealSet {
            key: key("tag"),
            values: [real(1.5), real(2.5)].into(),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_reports_unparseable_value_with_definition() {
    let definition = TagDefinition::IntegerSet(integer_definition());
    let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["text"])),
        Err(ContentDiagnostic::UnparseableTagValue {
            key: key("tag"),
            definition: Some(definition),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_reports_disallowed_value_with_definition() {
    let definition = TagDefinition::IntegerSet(integer_range(1, 2));
    let strict_schema = schema(false, vec![], vec![("tag", definition.clone())]);
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["3"])),
        Err(ContentDiagnostic::NotAllowedTagValue {
            key: key("tag"),
            definition,
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_defaults_undefined_values_to_text_set() {
    let additional = schema(true, vec![], vec![]);
    assert_eq!(
        additional.normalize_set_tag(key("extra"), &values(&["1", "2"])),
        Ok(Tag::TextSet {
            key: key("extra"),
            values: [
                TextTagValue::new("1").unwrap(),
                TextTagValue::new("2").unwrap()
            ]
            .into(),
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_omits_definition_for_invalid_undefined_value() {
    let additional = schema(true, vec![], vec![]);
    assert_eq!(
        additional.normalize_set_tag(key("extra"), &values(&[""])),
        Err(ContentDiagnostic::UnparseableTagValue {
            key: key("extra"),
            definition: None,
        })
    );
}

#[test]
fn v0_schema_normalize_set_tag_rejects_integer_values_equal_after_conversion() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::IntegerSet(integer_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["1", "01"])),
        Err(ContentDiagnostic::DuplicateSetValue { key: key("tag") })
    );
}

#[test]
fn v0_schema_normalize_set_tag_rejects_equivalent_real_spellings() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::RealSet(real_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["1.0", "1.00"])),
        Err(ContentDiagnostic::DuplicateSetValue { key: key("tag") })
    );
}

#[test]
fn v0_schema_normalize_set_tag_rejects_positive_and_negative_zero() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::RealSet(real_definition()))],
    );
    assert_eq!(
        strict_schema.normalize_set_tag(key("tag"), &values(&["0.0", "-0.0"])),
        Err(ContentDiagnostic::DuplicateSetValue { key: key("tag") })
    );
}

// === Search term construction ===

#[test]
fn v0_schema_construct_search_term_uses_defined_value_type_for_scalar_and_set() {
    for definition in [
        TagDefinition::Text(text_definition()),
        TagDefinition::TextSet(text_definition()),
    ] {
        let term = schema(false, vec![], vec![("tag", definition)])
            .construct_search_term(&key("tag"), &search_values(&["5"]));
        assert!(matches!(
            term,
            Some(Term::Tag {
                predicate: TagPredicate::TextMatch { candidates },
                ..
            }) if candidates.contains(&TextTagValue::new("5").unwrap())
        ));
    }

    for definition in [
        TagDefinition::Integer(integer_definition()),
        TagDefinition::IntegerSet(integer_definition()),
    ] {
        let term = schema(false, vec![], vec![("tag", definition)])
            .construct_search_term(&key("tag"), &search_values(&["05"]));
        assert!(matches!(
            term,
            Some(Term::Tag {
                predicate: TagPredicate::IntegerMatch { candidates },
                ..
            }) if candidates.contains(&integer(5))
        ));
    }

    for definition in [
        TagDefinition::Real(real_definition()),
        TagDefinition::RealSet(real_definition()),
    ] {
        let term = schema(false, vec![], vec![("tag", definition)])
            .construct_search_term(&key("tag"), &search_values(&["1.50"]));
        assert!(matches!(
            term,
            Some(Term::Tag {
                predicate: TagPredicate::RealMatch { candidates },
                ..
            }) if candidates.contains(&real(1.5))
        ));
    }
}

#[test]
fn v0_schema_construct_search_term_uses_text_for_allowed_undefined_key() {
    let term = schema(true, vec![], vec![])
        .construct_search_term(&key("undefined"), &search_values(&["5"]));
    assert!(matches!(
        term,
        Some(Term::Tag {
            predicate: TagPredicate::TextMatch { candidates },
            ..
        }) if candidates.contains(&TextTagValue::new("5").unwrap())
    ));
}

#[test]
fn v0_schema_construct_search_term_rejects_disallowed_key_and_key_only_definition() {
    let strict_schema = schema(false, vec![], vec![("key_only", TagDefinition::KeyOnly)]);
    assert!(
        strict_schema
            .construct_search_term(&key("undefined"), &search_values(&["value"]))
            .is_none()
    );
    assert!(
        strict_schema
            .construct_search_term(&key("key_only"), &search_values(&["value"]))
            .is_none()
    );
}

#[test]
fn v0_schema_construct_search_term_rejects_empty_or_invalid_candidates() {
    let strict_schema = schema(
        false,
        vec![],
        vec![("tag", TagDefinition::Integer(integer_range(1, 5)))],
    );
    for candidates in [
        search_values(&[]),
        search_values(&["1", "not an integer"]),
        search_values(&["1", "6"]),
    ] {
        assert!(
            strict_schema
                .construct_search_term(&key("tag"), &candidates)
                .is_none()
        );
    }

    assert!(
        schema(true, vec![], vec![])
            .construct_search_term(&key("undefined"), &search_values(&[""]))
            .is_none()
    );
}

// === each TagValueDefinition ===
// --- TextTagValueDefinition ---

#[test]
fn text_tag_value_definition_validate_format_accepts_valid_range() {
    assert!(text_range(1, 2).validate_format().is_ok());
}

#[test]
fn text_tag_value_definition_validate_format_rejects_reversed_range() {
    assert!(text_range(3, 2).validate_format().is_err());
}

#[test]
fn text_tag_value_definition_validate_format_rejects_allowed_values_with_range() {
    let mut definition = text_range(1, 2);
    definition.min_length = None;
    definition.allowed_values = Some([TextTagValue::new("a").unwrap()].into());
    assert!(definition.validate_format().is_err());

    definition.min_length = Some(NonZeroU8::new(1).unwrap());
    definition.max_length = None;
    assert!(definition.validate_format().is_err());
}

#[test]
fn text_tag_value_definition_validate_format_accepts_allowed_values_without_range() {
    let mut definition = text_definition();
    definition.allowed_values = Some([TextTagValue::new("a").unwrap()].into());
    assert!(definition.validate_format().is_ok());
}

#[test]
fn text_tag_value_definition_parse_accepts_value_at_range_boundary() {
    let definition = text_range(2, 2);
    assert_eq!(
        definition.parse(&"あい".to_string()),
        Ok(TextTagValue::new("あい").unwrap())
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_value_below_min_length() {
    let definition = text_range(2, 2);
    assert_eq!(
        definition.parse(&"あ".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_empty_value() {
    let definition = text_range(2, 2);
    assert_eq!(
        definition.parse(&"".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_value_above_global_length_limit() {
    let definition = text_definition();
    assert_eq!(
        definition.parse(&"あ".repeat(u8::MAX as usize + 1)),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_value_above_max_length() {
    let definition = text_range(2, 2);
    assert_eq!(
        definition.parse(&"あいう".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_non_nfc_value() {
    let definition = text_range(2, 2);
    assert_eq!(
        definition.parse(&"が".nfd().collect::<String>()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn text_tag_value_definition_parse_accepts_allowed_value() {
    let definition = TextTagValueDefinition {
        min_length: None,
        max_length: None,
        allowed_values: Some([TextTagValue::new("yes").unwrap()].into()),
    };
    assert_eq!(
        definition.parse(&"yes".to_string()),
        Ok(TextTagValue::new("yes").unwrap())
    );
}

#[test]
fn text_tag_value_definition_parse_rejects_disallowed_value() {
    let definition = TextTagValueDefinition {
        min_length: None,
        max_length: None,
        allowed_values: Some([TextTagValue::new("yes").unwrap()].into()),
    };
    assert_eq!(
        definition.parse(&"no".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

// --- IntegerTagValueDefinition ---

#[test]
fn integer_tag_value_definition_validate_format_accepts_valid_range() {
    assert!(integer_range(1, 2).validate_format().is_ok());
}

#[test]
fn integer_tag_value_definition_validate_format_rejects_reversed_range() {
    assert!(integer_range(3, 2).validate_format().is_err());
}

#[test]
fn integer_tag_value_definition_validate_format_rejects_allowed_values_with_range() {
    let mut definition = integer_range(1, 2);
    definition.min = None;
    definition.allowed_values = Some([integer(1)].into());
    assert!(definition.validate_format().is_err());

    definition.min = Some(integer(1));
    definition.max = None;
    assert!(definition.validate_format().is_err());
}

#[test]
fn integer_tag_value_definition_validate_format_accepts_allowed_values_without_range() {
    let mut definition = integer_definition();
    definition.allowed_values = Some([integer(1)].into());
    assert!(definition.validate_format().is_ok());
}

#[test]
fn integer_tag_value_definition_parse_accepts_values_at_range_boundaries() {
    let definition = integer_range(1, 2);
    assert_eq!(definition.parse(&"1".to_string()), Ok(integer(1)));
    assert_eq!(definition.parse(&"2".to_string()), Ok(integer(2)));
}

#[test]
fn integer_tag_value_definition_parse_rejects_value_below_min() {
    let definition = integer_range(1, 2);
    assert_eq!(
        definition.parse(&"0".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn integer_tag_value_definition_parse_rejects_value_above_max() {
    let definition = integer_range(1, 2);
    assert_eq!(
        definition.parse(&"3".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn integer_tag_value_definition_parse_rejects_non_integer_value() {
    let definition = integer_range(1, 2);
    assert_eq!(
        definition.parse(&"text".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn integer_tag_value_definition_parse_rejects_value_outside_safe_integer_range() {
    let definition = integer_range(1, 2);
    assert_eq!(
        definition.parse(&"9007199254740992".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn integer_tag_value_definition_parse_accepts_allowed_value() {
    let definition = IntegerTagValueDefinition {
        min: None,
        max: None,
        allowed_values: Some([integer(2)].into()),
    };
    assert_eq!(definition.parse(&"2".to_string()), Ok(integer(2)));
}

#[test]
fn integer_tag_value_definition_parse_rejects_disallowed_value() {
    let definition = IntegerTagValueDefinition {
        min: None,
        max: None,
        allowed_values: Some([integer(2)].into()),
    };
    assert_eq!(
        definition.parse(&"1".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

// --- RealTagValueDefinition ---

#[test]
fn real_tag_value_definition_validate_format_accepts_valid_range() {
    assert!(real_range(1.0, 2.0).validate_format().is_ok());
}

#[test]
fn real_tag_value_definition_validate_format_rejects_reversed_range() {
    assert!(real_range(3.0, 2.0).validate_format().is_err());
}

#[test]
fn real_tag_value_definition_validate_format_rejects_allowed_values_with_range() {
    let mut definition = real_range(1.0, 2.0);
    definition.min = None;
    definition.allowed_values = Some([real(1.0)].into());
    assert!(definition.validate_format().is_err());

    definition.min = Some(real(1.0));
    definition.max = None;
    assert!(definition.validate_format().is_err());
}

#[test]
fn real_tag_value_definition_validate_format_accepts_allowed_values_without_range() {
    let mut definition = real_definition();
    definition.allowed_values = Some([real(1.0)].into());
    assert!(definition.validate_format().is_ok());
}

#[test]
fn real_tag_value_definition_parse_accepts_values_at_range_boundaries() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(definition.parse(&"1.0".to_string()), Ok(real(1.0)));
    assert_eq!(definition.parse(&"2.0".to_string()), Ok(real(2.0)));
}

#[test]
fn real_tag_value_definition_parse_rejects_value_below_min() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(
        definition.parse(&"0.5".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn real_tag_value_definition_parse_rejects_value_above_max() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(
        definition.parse(&"2.5".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

#[test]
fn real_tag_value_definition_parse_rejects_non_numeric_value() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(
        definition.parse(&"text".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn real_tag_value_definition_parse_rejects_nan() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(
        definition.parse(&"NaN".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn real_tag_value_definition_parse_rejects_infinity() {
    let definition = real_range(1.0, 2.0);
    assert_eq!(
        definition.parse(&"inf".to_string()),
        Err(TagValueDefinitionViolation::Unparseable)
    );
}

#[test]
fn real_tag_value_definition_parse_accepts_allowed_value() {
    let definition = RealTagValueDefinition {
        min: None,
        max: None,
        allowed_values: Some([real(1.5)].into()),
    };
    assert_eq!(definition.parse(&"1.5".to_string()), Ok(real(1.5)));
}

#[test]
fn real_tag_value_definition_parse_rejects_disallowed_value() {
    let definition = RealTagValueDefinition {
        min: None,
        max: None,
        allowed_values: Some([real(1.5)].into()),
    };
    assert_eq!(
        definition.parse(&"2.5".to_string()),
        Err(TagValueDefinitionViolation::NotAllowedValue)
    );
}

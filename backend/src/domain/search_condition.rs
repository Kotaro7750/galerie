use std::collections::HashSet;

use crate::domain::{
    Content, MediaType,
    tag::{IntegerTagValue, RealTagValue, TagKey, TextTagValue},
};

#[derive(Debug)]
/// Represents a contents filter that consists of ANDed search terms
pub(crate) struct SearchCondition {
    terms: Vec<Term>,
}

impl SearchCondition {
    pub(crate) fn new(terms: &[Term]) -> Self {
        Self {
            terms: terms.to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
/// Represents single search term that can be used to filter contents
pub(crate) enum Term {
    FileFormat {
        predicate: FileFormatPredicate,
    },
    Tag {
        key: TagKey,
        predicate: TagPredicate,
    },
}

impl Term {
    pub(crate) fn new_file_format(predicate: FileFormatPredicate) -> Self {
        Self::FileFormat { predicate }
    }

    pub(crate) fn new_tag(key: TagKey, predicate: TagPredicate) -> Self {
        Self::Tag { key, predicate }
    }
}

#[derive(Debug, Clone)]
/// Search predicate for file format
pub(crate) enum FileFormatPredicate {
    /// Match when file format of the content is in candidates
    Match { candidates: HashSet<MediaType> },
}

impl FileFormatPredicate {
    pub(crate) fn new_match(media_types: &[MediaType]) -> Option<Self> {
        if media_types.is_empty() {
            None
        } else {
            Some(Self::Match {
                candidates: media_types.iter().cloned().collect(),
            })
        }
    }
}

#[derive(Debug, Clone)]
/// Search predicate for tag
pub(crate) enum TagPredicate {
    Exists,
    /// Match when content has specified key and tag value is in candidates
    TextMatch {
        candidates: HashSet<TextTagValue>,
    },
    IntegerMatch {
        candidates: HashSet<IntegerTagValue>,
    },
    RealMatch {
        candidates: HashSet<RealTagValue>,
    },
}

impl TagPredicate {
    pub(crate) fn new_text_match(tag_values: &[TextTagValue]) -> Option<Self> {
        if tag_values.is_empty() {
            None
        } else {
            Some(Self::TextMatch {
                candidates: tag_values.iter().cloned().collect(),
            })
        }
    }

    pub(crate) fn new_integer_match(tag_values: &[IntegerTagValue]) -> Option<Self> {
        if tag_values.is_empty() {
            None
        } else {
            Some(Self::IntegerMatch {
                candidates: tag_values.iter().cloned().collect(),
            })
        }
    }

    pub(crate) fn new_real_match(tag_values: &[RealTagValue]) -> Option<Self> {
        if tag_values.is_empty() {
            None
        } else {
            Some(Self::RealMatch {
                candidates: tag_values.iter().cloned().collect(),
            })
        }
    }
}

pub(crate) trait ContentConditionMatcher {
    /// Check if the content matches the condition
    /// Returns true if the content matches the condition, false otherwise
    fn is_match(&self, content: &Content) -> bool;
}

impl ContentConditionMatcher for SearchCondition {
    fn is_match(&self, content: &Content) -> bool {
        self.terms.iter().all(|term| term.is_match(content))
    }
}

impl ContentConditionMatcher for Term {
    fn is_match(&self, content: &Content) -> bool {
        match self {
            Term::FileFormat { predicate } => match predicate {
                FileFormatPredicate::Match { candidates } => {
                    candidates.contains(&content.media_type)
                }
            },
            Term::Tag { key, predicate } => match predicate {
                TagPredicate::Exists => content.tags.as_ref().contains_key(key),
                TagPredicate::TextMatch { candidates } => {
                    if let Some(tag_value) = content.tags.as_ref().get(key) {
                        candidates
                            .iter()
                            .any(|candidate| tag_value.contain_text_value(candidate))
                    } else {
                        false
                    }
                }
                TagPredicate::IntegerMatch { candidates } => {
                    if let Some(tag_value) = content.tags.as_ref().get(key) {
                        candidates
                            .iter()
                            .any(|candidate| tag_value.contain_integer_value(candidate))
                    } else {
                        false
                    }
                }
                TagPredicate::RealMatch { candidates } => {
                    if let Some(tag_value) = content.tags.as_ref().get(key) {
                        candidates
                            .iter()
                            .any(|candidate| tag_value.contain_real_value(candidate))
                    } else {
                        false
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use url::Url;
    use uuid::Uuid;

    use super::super::tag::*;
    use super::*;

    fn text_tag(key: &str, value: &str, is_set: bool) -> Tag {
        if is_set {
            Tag::TextSet {
                key: TagKey::new(key).unwrap(),
                values: vec![
                    TextTagValue::new(value).unwrap(),
                    TextTagValue::new(format!("{}1", value).as_str()).unwrap(),
                ]
                .into_iter()
                .collect(),
            }
        } else {
            Tag::Text {
                key: TagKey::new(key).unwrap(),
                value: TextTagValue::new(value).unwrap(),
            }
        }
    }

    #[test]
    fn test_content_media_type_match() {
        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&vec![]).unwrap(),
            HashSet::new(),
        );

        let media_type_term = Term::FileFormat {
            predicate: FileFormatPredicate::Match {
                candidates: vec![MediaType::Avif].into_iter().collect(),
            },
        };
        let empty_media_type_term = Term::FileFormat {
            predicate: FileFormatPredicate::Match {
                candidates: vec![].into_iter().collect(),
            },
        };

        assert!(media_type_term.is_match(&content));
        assert!(!empty_media_type_term.is_match(&content));
    }

    #[test]
    fn test_content_tag_existence_match() {
        let key = TagKey::new("exists").unwrap();

        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&vec![text_tag("exists", "value", false)]).unwrap(),
            HashSet::new(),
        );

        let tag_existence_term = Term::Tag {
            key: key.clone(),
            predicate: TagPredicate::Exists,
        };
        let tag_not_existence_term = Term::Tag {
            key: TagKey::new("not_exists").unwrap(),
            predicate: TagPredicate::Exists,
        };

        assert!(tag_existence_term.is_match(&content));
        assert!(!tag_not_existence_term.is_match(&content));
    }

    #[test]
    fn test_content_tag_value_match() {
        let key = TagKey::new("key").unwrap();
        let value = TextTagValue::new("value").unwrap();

        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&vec![text_tag("key", "value", false)]).unwrap(),
            HashSet::new(),
        );

        let tag_value_term = Term::Tag {
            key: key.clone(),
            predicate: TagPredicate::TextMatch {
                candidates: vec![value.clone()].into_iter().collect(),
            },
        };
        let tag_not_value_term = Term::Tag {
            key: key.clone(),
            predicate: TagPredicate::TextMatch {
                candidates: vec![TextTagValue::new("not_value").unwrap()]
                    .into_iter()
                    .collect(),
            },
        };

        assert!(tag_value_term.is_match(&content));
        assert!(!tag_not_value_term.is_match(&content));
    }

    #[test]
    fn test_content_integer_tag_value_match() {
        let key = TagKey::new("key").unwrap();
        let value = IntegerTagValue::new(42).unwrap();

        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&vec![Tag::Integer {
                key: key.clone(),
                value: value.clone(),
            }])
            .unwrap(),
            HashSet::new(),
        );

        let tag_value_term = Term::Tag {
            key: key.clone(),
            predicate: TagPredicate::IntegerMatch {
                candidates: vec![value].into_iter().collect(),
            },
        };
        let tag_not_value_term = Term::Tag {
            key,
            predicate: TagPredicate::IntegerMatch {
                candidates: vec![IntegerTagValue::new(43).unwrap()]
                    .into_iter()
                    .collect(),
            },
        };

        assert!(tag_value_term.is_match(&content));
        assert!(!tag_not_value_term.is_match(&content));
    }

    #[test]
    fn test_content_real_tag_value_match() {
        let key = TagKey::new("key").unwrap();
        let value = RealTagValue::new(1.5).unwrap();

        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&vec![Tag::Real {
                key: key.clone(),
                value: value.clone(),
            }])
            .unwrap(),
            HashSet::new(),
        );

        let tag_value_term = Term::Tag {
            key: key.clone(),
            predicate: TagPredicate::RealMatch {
                candidates: vec![value].into_iter().collect(),
            },
        };
        let tag_not_value_term = Term::Tag {
            key,
            predicate: TagPredicate::RealMatch {
                candidates: vec![RealTagValue::new(2.5).unwrap()].into_iter().collect(),
            },
        };

        assert!(tag_value_term.is_match(&content));
        assert!(!tag_not_value_term.is_match(&content));
    }

    #[test]
    fn test_content_set_tag_value_matches_only_same_type() {
        let content = Content::new(
            Uuid::new_v4().try_into().unwrap(),
            MediaType::Avif,
            Url::parse("https://example.com").unwrap(),
            Url::parse("https://example.com").unwrap(),
            TagSet::new(&[
                Tag::TextSet {
                    key: TagKey::new("text").unwrap(),
                    values: [TextTagValue::new("1").unwrap()].into(),
                },
                Tag::IntegerSet {
                    key: TagKey::new("integer").unwrap(),
                    values: [
                        IntegerTagValue::new(1).unwrap(),
                        IntegerTagValue::new(2).unwrap(),
                    ]
                    .into(),
                },
                Tag::RealSet {
                    key: TagKey::new("real").unwrap(),
                    values: [RealTagValue::new(1.0).unwrap()].into(),
                },
            ])
            .unwrap(),
            HashSet::new(),
        );

        let text_match = TagPredicate::new_text_match(&[TextTagValue::new("1").unwrap()]).unwrap();
        let integer_match = TagPredicate::new_integer_match(&[
            IntegerTagValue::new(3).unwrap(),
            IntegerTagValue::new(2).unwrap(),
        ])
        .unwrap();
        let real_match = TagPredicate::new_real_match(&[RealTagValue::new(1.0).unwrap()]).unwrap();

        for (name, predicate) in [
            ("text", text_match.clone()),
            ("integer", integer_match.clone()),
            ("real", real_match.clone()),
        ] {
            assert!(Term::new_tag(TagKey::new(name).unwrap(), predicate).is_match(&content));
        }

        assert!(!Term::new_tag(TagKey::new("text").unwrap(), integer_match).is_match(&content));
        assert!(!Term::new_tag(TagKey::new("integer").unwrap(), real_match).is_match(&content));
        assert!(!Term::new_tag(TagKey::new("real").unwrap(), text_match).is_match(&content));
    }
}

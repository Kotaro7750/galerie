#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
pub type SSE = std::pin::Pin<std::boxed::Box<dyn futures_util::Stream<Item = std::result::Result<axum::response::sse::Event, std::convert::Infallible>> + std::marker::Send + std::marker::Sync>>;

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
  let mut errs = validator::ValidationErrors::new();
  errs.add("na", e);
  errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(v: &std::collections::HashMap<String, T>) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}





    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct GetContentPathParams {
                #[validate(
                          regex(path = *RE_GETCONTENTPATHPARAMS_CONTENT_ID),
            )]
                pub content_id: String,
    }

    lazy_static::lazy_static! {
        static ref RE_GETCONTENTPATHPARAMS_CONTENT_ID: regex::Regex = regex::Regex::new("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
    }


    #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
    #[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
    pub struct ListContentsQueryParams {
            /// 一覧取得時のページネーションに用いるカーソル。 
                #[serde(rename = "cursor")]
                #[validate(
                        length(min = 1),
              )]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub cursor: Option<String>,
            /// 1ページに含めるコンテンツの最大件数。 継続リクエストでは直前のリクエストと異なる値を指定してもよい。 
                #[serde(rename = "limit")]
                #[validate(
                        range(min = 1u8, max = 100u8),
              )]
                    #[serde(skip_serializing_if="Option::is_none")]
                    pub limit: Option<u8>,
            /// 検索条件の項のJSON配列をURLエンコードした値。 省略または空配列は全件を表す。 
                #[serde(rename = "condition")]
                    #[serde(default)]
                    pub condition: Vec<models::SearchTerm>,
    }




/// 不正なリクエストを表すProblem Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BadRequestProblem {
    /// エラー種別を識別するURI参照
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// エラー種別の短い説明
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: i32,

    /// このエラーの具体的な説明
    #[serde(rename = "detail")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    /// このエラー発生を識別するURI参照
    #[serde(rename = "instance")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}



impl BadRequestProblem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> BadRequestProblem {
        BadRequestProblem {
 r_type: r#"about:blank"#.to_string(),
 title,
 status,
 detail: None,
 instance: None,
        }
    }
}

/// Converts the BadRequestProblem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for BadRequestProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a BadRequestProblem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for BadRequestProblem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing BadRequestProblem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing BadRequestProblem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(BadRequestProblem {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in BadRequestProblem".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in BadRequestProblem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in BadRequestProblem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<BadRequestProblem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<BadRequestProblem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<BadRequestProblem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for BadRequestProblem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<BadRequestProblem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <BadRequestProblem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into BadRequestProblem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// コンテンツファイルの情報
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Content {
    /// RFC 9562に準拠したUUID v4形式のコンテンツ識別子
    #[serde(rename = "id")]
    #[validate(
            regex(path = *RE_CONTENT_ID),
          custom(function = "check_xss_string"),
    )]
    pub id: String,

    #[serde(rename = "mediaType")]
          #[validate(nested)]
    pub media_type: models::MediaType,

    /// コンテンツ配信エンドポイントからコンテンツファイルを取得する絶対URL
    #[serde(rename = "contentUrl")]
    #[validate(
            regex(path = *RE_CONTENT_CONTENT_URL),
          custom(function = "check_xss_string"),
    )]
    pub content_url: String,

    /// サムネイル表示用のコンテンツファイルを取得する絶対URL
    #[serde(rename = "thumbnailUrl")]
    #[validate(
            regex(path = *RE_CONTENT_THUMBNAIL_URL),
          custom(function = "check_xss_string"),
    )]
    pub thumbnail_url: String,

    /// 有効なフォーマットかつスキーマが存在する場合にはスキーマと整合するタグ。 該当するタグがなければ空配列を返す。 同一コンテンツ内のタグのkeyは一意とし、値や型が異なる場合も同じkeyを持つタグの重複は認めない。 
    #[serde(rename = "tags")]
          #[validate(nested)]
    pub tags: Vec<models::Tag>,

    /// コンテンツのメタデータに関する診断情報。異常がない場合は空配列を返す。 単一タグに関する異常は同じkeyにつき最大1件とする。 異常のあるタグはtagsに含めず、タグ検索の対象にも含めない。  診断の種類（`kind`）: - `invalidKey`: XMPプロパティ名がタグキーの条件を満たさない - `duplicateKey`: 同じタグキーのXMPプロパティが複数存在する - `unsupportedXmpValueType`: XMPの値がタグとして対応しない形式である - `notAllowedTagKey`: タグキーがタグスキーマ上許容されない - `unparseableTagValue`: XMPの値をスキーマ定義またはフォールバックの型として解釈できない - `duplicateSetValue`: セットの要素値を解釈した結果、同じ値が複数存在する - `notAllowedTagValue`: 解釈したタグの値がスキーマの制約を満たさない - `missingRequiredTag`: 有効なタグの中に必須タグが存在しない  `definition`フィールドを含む場合、タグスキーマに記載されたタグ定義を示す。 
    #[serde(rename = "diagnostics")]
          #[validate(nested)]
    pub diagnostics: Vec<models::ContentDiagnostic>,

}


lazy_static::lazy_static! {
    static ref RE_CONTENT_ID: regex::Regex = regex::Regex::new("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-4[0-9a-fA-F]{3}-[89aAbB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_CONTENT_CONTENT_URL: regex::Regex = regex::Regex::new("^https?://").unwrap();
}
lazy_static::lazy_static! {
    static ref RE_CONTENT_THUMBNAIL_URL: regex::Regex = regex::Regex::new("^https?://").unwrap();
}

impl Content {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(id: String, media_type: models::MediaType, content_url: String, thumbnail_url: String, tags: Vec<models::Tag>, diagnostics: Vec<models::ContentDiagnostic>, ) -> Content {
        Content {
 id,
 media_type,
 content_url,
 thumbnail_url,
 tags,
 diagnostics,
        }
    }
}

/// Converts the Content value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),

            // Skipping mediaType in query parameter serialization


            Some("contentUrl".to_string()),
            Some(self.content_url.to_string()),


            Some("thumbnailUrl".to_string()),
            Some(self.thumbnail_url.to_string()),

            // Skipping tags in query parameter serialization

            // Skipping diagnostics in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Content value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Content {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<String>,
            pub media_type: Vec<models::MediaType>,
            pub content_url: Vec<String>,
            pub thumbnail_url: Vec<String>,
            pub tags: Vec<Vec<models::Tag>>,
            pub diagnostics: Vec<Vec<models::ContentDiagnostic>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Content".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "mediaType" => intermediate_rep.media_type.push(<models::MediaType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contentUrl" => intermediate_rep.content_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "thumbnailUrl" => intermediate_rep.thumbnail_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "tags" => return std::result::Result::Err("Parsing a container in this style is not supported in Content".to_string()),
                    "diagnostics" => return std::result::Result::Err("Parsing a container in this style is not supported in Content".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Content".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Content {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in Content".to_string())?,
            media_type: intermediate_rep.media_type.into_iter().next().ok_or_else(|| "mediaType missing in Content".to_string())?,
            content_url: intermediate_rep.content_url.into_iter().next().ok_or_else(|| "contentUrl missing in Content".to_string())?,
            thumbnail_url: intermediate_rep.thumbnail_url.into_iter().next().ok_or_else(|| "thumbnailUrl missing in Content".to_string())?,
            tags: intermediate_rep.tags.into_iter().next().ok_or_else(|| "tags missing in Content".to_string())?,
            diagnostics: intermediate_rep.diagnostics.into_iter().next().ok_or_else(|| "diagnostics missing in Content".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Content> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Content>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Content>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Content - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Content> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Content as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Content - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// クライアントに設定されたコンテンツアクセス設定の有効期限
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentAccessConfiguration {
    /// この時刻以降はコンテンツアクセス設定の有効性を保証しない。有効期限がない場合は省略する。
    #[serde(rename = "invalidAfter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub invalid_after: Option<chrono::DateTime::<chrono::Utc>>,

}



impl ContentAccessConfiguration {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> ContentAccessConfiguration {
        ContentAccessConfiguration {
 invalid_after: None,
        }
    }
}

/// Converts the ContentAccessConfiguration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ContentAccessConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping invalidAfter in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentAccessConfiguration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentAccessConfiguration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub invalid_after: Vec<chrono::DateTime::<chrono::Utc>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContentAccessConfiguration".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "invalidAfter" => intermediate_rep.invalid_after.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContentAccessConfiguration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContentAccessConfiguration {
            invalid_after: intermediate_rep.invalid_after.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContentAccessConfiguration> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ContentAccessConfiguration>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContentAccessConfiguration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ContentAccessConfiguration - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ContentAccessConfiguration> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContentAccessConfiguration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ContentAccessConfiguration - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum ContentDiagnostic {
    InvalidKeyDiagnostic(models::InvalidKeyDiagnostic),
    DuplicateKeyDiagnostic(models::DuplicateKeyDiagnostic),
    UnsupportedXmpValueTypeDiagnostic(models::UnsupportedXmpValueTypeDiagnostic),
    NotAllowedTagKeyDiagnostic(models::NotAllowedTagKeyDiagnostic),
    UnparseableTagValueDiagnostic(models::UnparseableTagValueDiagnostic),
    DuplicateSetValueDiagnostic(models::DuplicateSetValueDiagnostic),
    NotAllowedTagValueDiagnostic(models::NotAllowedTagValueDiagnostic),
    MissingRequiredTagDiagnostic(models::MissingRequiredTagDiagnostic),
}

impl validator::Validate for ContentDiagnostic
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::InvalidKeyDiagnostic(v) => v.validate(),
            Self::DuplicateKeyDiagnostic(v) => v.validate(),
            Self::UnsupportedXmpValueTypeDiagnostic(v) => v.validate(),
            Self::NotAllowedTagKeyDiagnostic(v) => v.validate(),
            Self::UnparseableTagValueDiagnostic(v) => v.validate(),
            Self::DuplicateSetValueDiagnostic(v) => v.validate(),
            Self::NotAllowedTagValueDiagnostic(v) => v.validate(),
            Self::MissingRequiredTagDiagnostic(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentDiagnostic {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::InvalidKeyDiagnostic> for ContentDiagnostic {
    fn from(value: models::InvalidKeyDiagnostic) -> Self {
        Self::InvalidKeyDiagnostic(value)
    }
}
impl From<models::DuplicateKeyDiagnostic> for ContentDiagnostic {
    fn from(value: models::DuplicateKeyDiagnostic) -> Self {
        Self::DuplicateKeyDiagnostic(value)
    }
}
impl From<models::UnsupportedXmpValueTypeDiagnostic> for ContentDiagnostic {
    fn from(value: models::UnsupportedXmpValueTypeDiagnostic) -> Self {
        Self::UnsupportedXmpValueTypeDiagnostic(value)
    }
}
impl From<models::NotAllowedTagKeyDiagnostic> for ContentDiagnostic {
    fn from(value: models::NotAllowedTagKeyDiagnostic) -> Self {
        Self::NotAllowedTagKeyDiagnostic(value)
    }
}
impl From<models::UnparseableTagValueDiagnostic> for ContentDiagnostic {
    fn from(value: models::UnparseableTagValueDiagnostic) -> Self {
        Self::UnparseableTagValueDiagnostic(value)
    }
}
impl From<models::DuplicateSetValueDiagnostic> for ContentDiagnostic {
    fn from(value: models::DuplicateSetValueDiagnostic) -> Self {
        Self::DuplicateSetValueDiagnostic(value)
    }
}
impl From<models::NotAllowedTagValueDiagnostic> for ContentDiagnostic {
    fn from(value: models::NotAllowedTagValueDiagnostic) -> Self {
        Self::NotAllowedTagValueDiagnostic(value)
    }
}
impl From<models::MissingRequiredTagDiagnostic> for ContentDiagnostic {
    fn from(value: models::MissingRequiredTagDiagnostic) -> Self {
        Self::MissingRequiredTagDiagnostic(value)
    }
}





/// RFC 9562に準拠したUUID v4形式のコンテンツ識別子
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentId(pub String);

impl validator::Validate for ContentId {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for ContentId {
    fn from(x: String) -> Self {
        ContentId(x)
    }
}

impl std::fmt::Display for ContentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ContentId {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(ContentId(x.to_string()))
    }
}

impl std::convert::From<ContentId> for String {
    fn from(x: ContentId) -> Self {
        x.0
    }
}

impl std::ops::Deref for ContentId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for ContentId {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// コンテンツに設定するメタデータ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentMetadataInput {
    /// 設定するタグ。タグがない場合は空配列を指定する。
    #[serde(rename = "tags")]
          #[validate(nested)]
    pub tags: Vec<models::Tag>,

}



impl ContentMetadataInput {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tags: Vec<models::Tag>, ) -> ContentMetadataInput {
        ContentMetadataInput {
 tags,
        }
    }
}

/// Converts the ContentMetadataInput value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ContentMetadataInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping tags in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentMetadataInput value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentMetadataInput {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tags: Vec<Vec<models::Tag>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContentMetadataInput".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "tags" => return std::result::Result::Err("Parsing a container in this style is not supported in ContentMetadataInput".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContentMetadataInput".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContentMetadataInput {
            tags: intermediate_rep.tags.into_iter().next().ok_or_else(|| "tags missing in ContentMetadataInput".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContentMetadataInput> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ContentMetadataInput>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContentMetadataInput>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ContentMetadataInput - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ContentMetadataInput> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContentMetadataInput as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ContentMetadataInput - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// コンテンツが存在しない、または必須のXMPを読み込み・解析できないことを表すProblem Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentNotFoundProblem {
    /// エラー種別を識別するURI参照
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// エラー種別の短い説明
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: i32,

    /// このエラーの具体的な説明
    #[serde(rename = "detail")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    /// このエラー発生を識別するURI参照
    #[serde(rename = "instance")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}



impl ContentNotFoundProblem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> ContentNotFoundProblem {
        ContentNotFoundProblem {
 r_type: r#"about:blank"#.to_string(),
 title,
 status,
 detail: None,
 instance: None,
        }
    }
}

/// Converts the ContentNotFoundProblem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ContentNotFoundProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentNotFoundProblem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentNotFoundProblem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContentNotFoundProblem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContentNotFoundProblem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContentNotFoundProblem {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in ContentNotFoundProblem".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in ContentNotFoundProblem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in ContentNotFoundProblem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContentNotFoundProblem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ContentNotFoundProblem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContentNotFoundProblem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ContentNotFoundProblem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ContentNotFoundProblem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContentNotFoundProblem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ContentNotFoundProblem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// カーソル方式でページングされたコンテンツ一覧
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ContentPage {
    /// このページに含まれるコンテンツ
    #[serde(rename = "items")]
          #[validate(nested)]
    pub items: Vec<models::Content>,

    /// 次ページを取得するためのカーソル。 最終ページではこのフィールドを返さない。 
    #[serde(rename = "nextCursor")]
    #[validate(
            length(min = 1),
          custom(function = "check_xss_string"),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_cursor: Option<String>,

}



impl ContentPage {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(items: Vec<models::Content>, ) -> ContentPage {
        ContentPage {
 items,
 next_cursor: None,
        }
    }
}

/// Converts the ContentPage value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ContentPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping items in query parameter serialization


            self.next_cursor.as_ref().map(|next_cursor| {
                [
                    "nextCursor".to_string(),
                    next_cursor.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ContentPage value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ContentPage {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub items: Vec<Vec<models::Content>>,
            pub next_cursor: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ContentPage".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "items" => return std::result::Result::Err("Parsing a container in this style is not supported in ContentPage".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "nextCursor" => intermediate_rep.next_cursor.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ContentPage".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ContentPage {
            items: intermediate_rep.items.into_iter().next().ok_or_else(|| "items missing in ContentPage".to_string())?,
            next_cursor: intermediate_rep.next_cursor.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ContentPage> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ContentPage>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ContentPage>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for ContentPage - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ContentPage> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ContentPage as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into ContentPage - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DuplicateKeyDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::DuplicateKeyDiagnosticKind,

}



impl DuplicateKeyDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::DuplicateKeyDiagnosticKind, ) -> DuplicateKeyDiagnostic {
        DuplicateKeyDiagnostic {
 key,
 kind,
        }
    }
}

/// Converts the DuplicateKeyDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DuplicateKeyDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DuplicateKeyDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DuplicateKeyDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::DuplicateKeyDiagnosticKind>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DuplicateKeyDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::DuplicateKeyDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DuplicateKeyDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DuplicateKeyDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in DuplicateKeyDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in DuplicateKeyDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DuplicateKeyDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DuplicateKeyDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DuplicateKeyDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for DuplicateKeyDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DuplicateKeyDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DuplicateKeyDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into DuplicateKeyDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DuplicateKeyDiagnosticKind {
    #[serde(rename = "duplicateKey")]
    DuplicateKey,
}

impl validator::Validate for DuplicateKeyDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for DuplicateKeyDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DuplicateKeyDiagnosticKind::DuplicateKey => write!(f, "duplicateKey"),
        }
    }
}

impl std::str::FromStr for DuplicateKeyDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "duplicateKey" => std::result::Result::Ok(DuplicateKeyDiagnosticKind::DuplicateKey),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DuplicateSetValueDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::DuplicateSetValueDiagnosticKind,

}



impl DuplicateSetValueDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::DuplicateSetValueDiagnosticKind, ) -> DuplicateSetValueDiagnostic {
        DuplicateSetValueDiagnostic {
 key,
 kind,
        }
    }
}

/// Converts the DuplicateSetValueDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for DuplicateSetValueDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DuplicateSetValueDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DuplicateSetValueDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::DuplicateSetValueDiagnosticKind>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DuplicateSetValueDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::DuplicateSetValueDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DuplicateSetValueDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DuplicateSetValueDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in DuplicateSetValueDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in DuplicateSetValueDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DuplicateSetValueDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<DuplicateSetValueDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DuplicateSetValueDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for DuplicateSetValueDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<DuplicateSetValueDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DuplicateSetValueDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into DuplicateSetValueDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum DuplicateSetValueDiagnosticKind {
    #[serde(rename = "duplicateSetValue")]
    DuplicateSetValue,
}

impl validator::Validate for DuplicateSetValueDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for DuplicateSetValueDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DuplicateSetValueDiagnosticKind::DuplicateSetValue => write!(f, "duplicateSetValue"),
        }
    }
}

impl std::str::FromStr for DuplicateSetValueDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "duplicateSetValue" => std::result::Result::Ok(DuplicateSetValueDiagnosticKind::DuplicateSetValue),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// Integer値からなる集合のタグ。要素は順序を持たず、値の重複は許可しない。JSONでは配列として表現する
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerSetTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::IntegerSetTagType,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::IntegerTagValue>,

}



impl IntegerSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::IntegerSetTagType, values: Vec<models::IntegerTagValue>, ) -> IntegerSetTag {
        IntegerSetTag {
 key,
 r_type,
 values,
        }
    }
}

/// Converts the IntegerSetTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for IntegerSetTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("values".to_string()),
            Some(self.values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IntegerSetTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IntegerSetTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::IntegerSetTagType>,
            pub values: Vec<Vec<models::IntegerTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing IntegerSetTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::IntegerSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in IntegerSetTag".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing IntegerSetTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IntegerSetTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in IntegerSetTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in IntegerSetTag".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in IntegerSetTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IntegerSetTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<IntegerSetTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<IntegerSetTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for IntegerSetTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<IntegerSetTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <IntegerSetTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into IntegerSetTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Integer値からなる集合の定義。allowedValuesとmin・maxは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerSetTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::IntegerSetTagType,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "min")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<i64>,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "max")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<i64>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::IntegerTagValue>>,

}



impl IntegerSetTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::IntegerSetTagType, ) -> IntegerSetTagDefinition {
        IntegerSetTagDefinition {
 key,
 r_type,
 min: None,
 max: None,
 allowed_values: None,
        }
    }
}

/// Converts the IntegerSetTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for IntegerSetTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min.as_ref().map(|min| {
                [
                    "min".to_string(),
                    min.to_string(),
                ].join(",")
            }),


            self.max.as_ref().map(|max| {
                [
                    "max".to_string(),
                    max.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IntegerSetTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IntegerSetTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::IntegerSetTagType>,
            pub min: Vec<i64>,
            pub max: Vec<i64>,
            pub allowed_values: Vec<Vec<models::IntegerTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing IntegerSetTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::IntegerSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min" => intermediate_rep.min.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max" => intermediate_rep.max.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in IntegerSetTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing IntegerSetTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IntegerSetTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in IntegerSetTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in IntegerSetTagDefinition".to_string())?,
            min: intermediate_rep.min.into_iter().next(),
            max: intermediate_rep.max.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IntegerSetTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<IntegerSetTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<IntegerSetTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for IntegerSetTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<IntegerSetTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <IntegerSetTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into IntegerSetTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum IntegerSetTagType {
    #[serde(rename = "integerSet")]
    IntegerSet,
}

impl validator::Validate for IntegerSetTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for IntegerSetTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IntegerSetTagType::IntegerSet => write!(f, "integerSet"),
        }
    }
}

impl std::str::FromStr for IntegerSetTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "integerSet" => std::result::Result::Ok(IntegerSetTagType::IntegerSet),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 単一のInteger値を持つタグ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::IntegerTagType,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "value")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    pub value: i64,

}



impl IntegerTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::IntegerTagType, value: i64, ) -> IntegerTag {
        IntegerTag {
 key,
 r_type,
 value,
        }
    }
}

/// Converts the IntegerTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for IntegerTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IntegerTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IntegerTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::IntegerTagType>,
            pub value: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing IntegerTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::IntegerTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing IntegerTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IntegerTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in IntegerTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in IntegerTag".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in IntegerTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IntegerTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<IntegerTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<IntegerTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for IntegerTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<IntegerTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <IntegerTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into IntegerTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// 単一のInteger値の定義。allowedValuesとmin・maxは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::IntegerTagType,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "min")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<i64>,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "max")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<i64>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::IntegerTagValue>>,

}



impl IntegerTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::IntegerTagType, ) -> IntegerTagDefinition {
        IntegerTagDefinition {
 key,
 r_type,
 min: None,
 max: None,
 allowed_values: None,
        }
    }
}

/// Converts the IntegerTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for IntegerTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min.as_ref().map(|min| {
                [
                    "min".to_string(),
                    min.to_string(),
                ].join(",")
            }),


            self.max.as_ref().map(|max| {
                [
                    "max".to_string(),
                    max.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a IntegerTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for IntegerTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::IntegerTagType>,
            pub min: Vec<i64>,
            pub max: Vec<i64>,
            pub allowed_values: Vec<Vec<models::IntegerTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing IntegerTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::IntegerTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min" => intermediate_rep.min.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max" => intermediate_rep.max.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in IntegerTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing IntegerTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(IntegerTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in IntegerTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in IntegerTagDefinition".to_string())?,
            min: intermediate_rep.min.into_iter().next(),
            max: intermediate_rep.max.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<IntegerTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<IntegerTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<IntegerTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for IntegerTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<IntegerTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <IntegerTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into IntegerTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum IntegerTagType {
    #[serde(rename = "integer")]
    Integer,
}

impl validator::Validate for IntegerTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for IntegerTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            IntegerTagType::Integer => write!(f, "integer"),
        }
    }
}

impl std::str::FromStr for IntegerTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "integer" => std::result::Result::Ok(IntegerTagType::Integer),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct IntegerTagValue(pub i64);

impl validator::Validate for IntegerTagValue {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<i64> for IntegerTagValue {
    fn from(x: i64) -> Self {
        IntegerTagValue(x)
    }
}

impl std::convert::From<IntegerTagValue> for i64 {
    fn from(x: IntegerTagValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for IntegerTagValue {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl std::ops::DerefMut for IntegerTagValue {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}



/// サーバー内部エラーを表すProblem Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InternalServerErrorProblem {
    /// エラー種別を識別するURI参照
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// エラー種別の短い説明
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: i32,

    /// このエラーの具体的な説明
    #[serde(rename = "detail")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    /// このエラー発生を識別するURI参照
    #[serde(rename = "instance")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}



impl InternalServerErrorProblem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> InternalServerErrorProblem {
        InternalServerErrorProblem {
 r_type: r#"about:blank"#.to_string(),
 title,
 status,
 detail: None,
 instance: None,
        }
    }
}

/// Converts the InternalServerErrorProblem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InternalServerErrorProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InternalServerErrorProblem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InternalServerErrorProblem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InternalServerErrorProblem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InternalServerErrorProblem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InternalServerErrorProblem {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in InternalServerErrorProblem".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in InternalServerErrorProblem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in InternalServerErrorProblem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InternalServerErrorProblem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InternalServerErrorProblem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InternalServerErrorProblem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InternalServerErrorProblem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InternalServerErrorProblem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InternalServerErrorProblem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InternalServerErrorProblem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InvalidKeyDiagnostic {
    /// 不正なXMPプロパティ名をそのまま返す
    #[serde(rename = "key")]
          #[validate(custom(function = "check_xss_string"))]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::InvalidKeyDiagnosticKind,

}



impl InvalidKeyDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::InvalidKeyDiagnosticKind, ) -> InvalidKeyDiagnostic {
        InvalidKeyDiagnostic {
 key,
 kind,
        }
    }
}

/// Converts the InvalidKeyDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InvalidKeyDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InvalidKeyDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InvalidKeyDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::InvalidKeyDiagnosticKind>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InvalidKeyDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::InvalidKeyDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InvalidKeyDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InvalidKeyDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in InvalidKeyDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in InvalidKeyDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InvalidKeyDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InvalidKeyDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InvalidKeyDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InvalidKeyDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InvalidKeyDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InvalidKeyDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InvalidKeyDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum InvalidKeyDiagnosticKind {
    #[serde(rename = "invalidKey")]
    InvalidKey,
}

impl validator::Validate for InvalidKeyDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for InvalidKeyDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InvalidKeyDiagnosticKind::InvalidKey => write!(f, "invalidKey"),
        }
    }
}

impl std::str::FromStr for InvalidKeyDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "invalidKey" => std::result::Result::Ok(InvalidKeyDiagnosticKind::InvalidKey),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// keyのみのタグ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct KeyOnlyTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::KeyOnlyTagType,

}



impl KeyOnlyTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::KeyOnlyTagType, ) -> KeyOnlyTag {
        KeyOnlyTag {
 key,
 r_type,
        }
    }
}

/// Converts the KeyOnlyTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for KeyOnlyTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a KeyOnlyTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for KeyOnlyTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::KeyOnlyTagType>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing KeyOnlyTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::KeyOnlyTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing KeyOnlyTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(KeyOnlyTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in KeyOnlyTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in KeyOnlyTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<KeyOnlyTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<KeyOnlyTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<KeyOnlyTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for KeyOnlyTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<KeyOnlyTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <KeyOnlyTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into KeyOnlyTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct KeyOnlyTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::KeyOnlyTagType,

}



impl KeyOnlyTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::KeyOnlyTagType, ) -> KeyOnlyTagDefinition {
        KeyOnlyTagDefinition {
 key,
 r_type,
        }
    }
}

/// Converts the KeyOnlyTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for KeyOnlyTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a KeyOnlyTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for KeyOnlyTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::KeyOnlyTagType>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing KeyOnlyTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::KeyOnlyTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing KeyOnlyTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(KeyOnlyTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in KeyOnlyTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in KeyOnlyTagDefinition".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<KeyOnlyTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<KeyOnlyTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<KeyOnlyTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for KeyOnlyTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<KeyOnlyTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <KeyOnlyTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into KeyOnlyTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum KeyOnlyTagType {
    #[serde(rename = "keyOnly")]
    KeyOnly,
}

impl validator::Validate for KeyOnlyTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for KeyOnlyTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            KeyOnlyTagType::KeyOnly => write!(f, "keyOnly"),
        }
    }
}

impl std::str::FromStr for KeyOnlyTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "keyOnly" => std::result::Result::Ok(KeyOnlyTagType::KeyOnly),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// コンテンツファイルのメディアタイプ
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MediaType {
    #[serde(rename = "image/avif")]
    ImageSlashAvif,
}

impl validator::Validate for MediaType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MediaType::ImageSlashAvif => write!(f, "image/avif"),
        }
    }
}

impl std::str::FromStr for MediaType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "image/avif" => std::result::Result::Ok(MediaType::ImageSlashAvif),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MediaTypeMatchKind {
    #[serde(rename = "mediaTypeMatch")]
    MediaTypeMatch,
}

impl validator::Validate for MediaTypeMatchKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for MediaTypeMatchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MediaTypeMatchKind::MediaTypeMatch => write!(f, "mediaTypeMatch"),
        }
    }
}

impl std::str::FromStr for MediaTypeMatchKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "mediaTypeMatch" => std::result::Result::Ok(MediaTypeMatchKind::MediaTypeMatch),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// コンテンツファイルフォーマットが指定値のうちいずれかに一致する場合にマッチする
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MediaTypeMatchTerm {
    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::MediaTypeMatchKind,

    #[serde(rename = "values")]
    #[validate(
            length(min = 1),
          nested,
    )]
    pub values: Vec<models::MediaType>,

}



impl MediaTypeMatchTerm {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(kind: models::MediaTypeMatchKind, values: Vec<models::MediaType>, ) -> MediaTypeMatchTerm {
        MediaTypeMatchTerm {
 kind,
 values,
        }
    }
}

/// Converts the MediaTypeMatchTerm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MediaTypeMatchTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping kind in query parameter serialization

            // Skipping values in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MediaTypeMatchTerm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MediaTypeMatchTerm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub kind: Vec<models::MediaTypeMatchKind>,
            pub values: Vec<Vec<models::MediaType>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MediaTypeMatchTerm".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::MediaTypeMatchKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in MediaTypeMatchTerm".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing MediaTypeMatchTerm".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MediaTypeMatchTerm {
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in MediaTypeMatchTerm".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in MediaTypeMatchTerm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MediaTypeMatchTerm> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MediaTypeMatchTerm>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MediaTypeMatchTerm>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MediaTypeMatchTerm - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MediaTypeMatchTerm> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MediaTypeMatchTerm as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MediaTypeMatchTerm - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MissingRequiredTagDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::MissingRequiredTagDiagnosticKind,

    #[serde(rename = "definition")]
          #[validate(nested)]
    pub definition: models::TagDefinition,

}



impl MissingRequiredTagDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::MissingRequiredTagDiagnosticKind, definition: models::TagDefinition, ) -> MissingRequiredTagDiagnostic {
        MissingRequiredTagDiagnostic {
 key,
 kind,
 definition,
        }
    }
}

/// Converts the MissingRequiredTagDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for MissingRequiredTagDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

            // Skipping definition in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MissingRequiredTagDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MissingRequiredTagDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::MissingRequiredTagDiagnosticKind>,
            pub definition: Vec<models::TagDefinition>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MissingRequiredTagDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::MissingRequiredTagDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "definition" => intermediate_rep.definition.push(<models::TagDefinition as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MissingRequiredTagDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MissingRequiredTagDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in MissingRequiredTagDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in MissingRequiredTagDiagnostic".to_string())?,
            definition: intermediate_rep.definition.into_iter().next().ok_or_else(|| "definition missing in MissingRequiredTagDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MissingRequiredTagDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<MissingRequiredTagDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MissingRequiredTagDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for MissingRequiredTagDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<MissingRequiredTagDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MissingRequiredTagDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into MissingRequiredTagDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum MissingRequiredTagDiagnosticKind {
    #[serde(rename = "missingRequiredTag")]
    MissingRequiredTag,
}

impl validator::Validate for MissingRequiredTagDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for MissingRequiredTagDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MissingRequiredTagDiagnosticKind::MissingRequiredTag => write!(f, "missingRequiredTag"),
        }
    }
}

impl std::str::FromStr for MissingRequiredTagDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "missingRequiredTag" => std::result::Result::Ok(MissingRequiredTagDiagnosticKind::MissingRequiredTag),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotAllowedTagKeyDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::NotAllowedTagKeyDiagnosticKind,

}



impl NotAllowedTagKeyDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::NotAllowedTagKeyDiagnosticKind, ) -> NotAllowedTagKeyDiagnostic {
        NotAllowedTagKeyDiagnostic {
 key,
 kind,
        }
    }
}

/// Converts the NotAllowedTagKeyDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NotAllowedTagKeyDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotAllowedTagKeyDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotAllowedTagKeyDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::NotAllowedTagKeyDiagnosticKind>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotAllowedTagKeyDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::NotAllowedTagKeyDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotAllowedTagKeyDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotAllowedTagKeyDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in NotAllowedTagKeyDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in NotAllowedTagKeyDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NotAllowedTagKeyDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NotAllowedTagKeyDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotAllowedTagKeyDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NotAllowedTagKeyDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NotAllowedTagKeyDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotAllowedTagKeyDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NotAllowedTagKeyDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum NotAllowedTagKeyDiagnosticKind {
    #[serde(rename = "notAllowedTagKey")]
    NotAllowedTagKey,
}

impl validator::Validate for NotAllowedTagKeyDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for NotAllowedTagKeyDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NotAllowedTagKeyDiagnosticKind::NotAllowedTagKey => write!(f, "notAllowedTagKey"),
        }
    }
}

impl std::str::FromStr for NotAllowedTagKeyDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "notAllowedTagKey" => std::result::Result::Ok(NotAllowedTagKeyDiagnosticKind::NotAllowedTagKey),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct NotAllowedTagValueDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::NotAllowedTagValueDiagnosticKind,

    #[serde(rename = "definition")]
          #[validate(nested)]
    pub definition: models::TagDefinition,

}



impl NotAllowedTagValueDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::NotAllowedTagValueDiagnosticKind, definition: models::TagDefinition, ) -> NotAllowedTagValueDiagnostic {
        NotAllowedTagValueDiagnostic {
 key,
 kind,
 definition,
        }
    }
}

/// Converts the NotAllowedTagValueDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for NotAllowedTagValueDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

            // Skipping definition in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a NotAllowedTagValueDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for NotAllowedTagValueDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::NotAllowedTagValueDiagnosticKind>,
            pub definition: Vec<models::TagDefinition>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing NotAllowedTagValueDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::NotAllowedTagValueDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "definition" => intermediate_rep.definition.push(<models::TagDefinition as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing NotAllowedTagValueDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(NotAllowedTagValueDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in NotAllowedTagValueDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in NotAllowedTagValueDiagnostic".to_string())?,
            definition: intermediate_rep.definition.into_iter().next().ok_or_else(|| "definition missing in NotAllowedTagValueDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<NotAllowedTagValueDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<NotAllowedTagValueDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<NotAllowedTagValueDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for NotAllowedTagValueDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<NotAllowedTagValueDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <NotAllowedTagValueDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into NotAllowedTagValueDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum NotAllowedTagValueDiagnosticKind {
    #[serde(rename = "notAllowedTagValue")]
    NotAllowedTagValue,
}

impl validator::Validate for NotAllowedTagValueDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for NotAllowedTagValueDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            NotAllowedTagValueDiagnosticKind::NotAllowedTagValue => write!(f, "notAllowedTagValue"),
        }
    }
}

impl std::str::FromStr for NotAllowedTagValueDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "notAllowedTagValue" => std::result::Result::Ok(NotAllowedTagValueDiagnosticKind::NotAllowedTagValue),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// RFC 9457に準拠したAPIエラーの詳細
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Problem {
    /// エラー種別を識別するURI参照
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// エラー種別の短い説明
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// HTTPステータスコード
    #[serde(rename = "status")]
    pub status: i32,

    /// このエラーの具体的な説明
    #[serde(rename = "detail")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    /// このエラー発生を識別するURI参照
    #[serde(rename = "instance")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}



impl Problem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> Problem {
        Problem {
 r_type: r#"about:blank"#.to_string(),
 title,
 status,
 detail: None,
 instance: None,
        }
    }
}

/// Converts the Problem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Problem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Problem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Problem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Problem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Problem {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in Problem".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in Problem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in Problem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Problem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<Problem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Problem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for Problem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<Problem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Problem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into Problem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Real値からなる集合のタグ。要素は順序を持たず、値の重複は許可しない。JSONでは配列として表現する
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RealSetTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::RealSetTagType,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::RealTagValue>,

}



impl RealSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::RealSetTagType, values: Vec<models::RealTagValue>, ) -> RealSetTag {
        RealSetTag {
 key,
 r_type,
 values,
        }
    }
}

/// Converts the RealSetTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RealSetTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("values".to_string()),
            Some(self.values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RealSetTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RealSetTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::RealSetTagType>,
            pub values: Vec<Vec<models::RealTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RealSetTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::RealSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in RealSetTag".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RealSetTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RealSetTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in RealSetTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in RealSetTag".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in RealSetTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RealSetTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RealSetTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RealSetTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for RealSetTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RealSetTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RealSetTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into RealSetTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Real値からなる集合の定義。allowedValuesとmin・maxは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RealSetTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::RealSetTagType,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "min")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<f64>,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "max")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<f64>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::RealTagValue>>,

}



impl RealSetTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::RealSetTagType, ) -> RealSetTagDefinition {
        RealSetTagDefinition {
 key,
 r_type,
 min: None,
 max: None,
 allowed_values: None,
        }
    }
}

/// Converts the RealSetTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RealSetTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min.as_ref().map(|min| {
                [
                    "min".to_string(),
                    min.to_string(),
                ].join(",")
            }),


            self.max.as_ref().map(|max| {
                [
                    "max".to_string(),
                    max.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RealSetTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RealSetTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::RealSetTagType>,
            pub min: Vec<f64>,
            pub max: Vec<f64>,
            pub allowed_values: Vec<Vec<models::RealTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RealSetTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::RealSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min" => intermediate_rep.min.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max" => intermediate_rep.max.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in RealSetTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RealSetTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RealSetTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in RealSetTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in RealSetTagDefinition".to_string())?,
            min: intermediate_rep.min.into_iter().next(),
            max: intermediate_rep.max.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RealSetTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RealSetTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RealSetTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for RealSetTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RealSetTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RealSetTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into RealSetTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum RealSetTagType {
    #[serde(rename = "realSet")]
    RealSet,
}

impl validator::Validate for RealSetTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for RealSetTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RealSetTagType::RealSet => write!(f, "realSet"),
        }
    }
}

impl std::str::FromStr for RealSetTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "realSet" => std::result::Result::Ok(RealSetTagType::RealSet),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 単一のReal値を持つタグ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RealTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::RealTagType,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "value")]
    pub value: f64,

}



impl RealTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::RealTagType, value: f64, ) -> RealTag {
        RealTag {
 key,
 r_type,
 value,
        }
    }
}

/// Converts the RealTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RealTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RealTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RealTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::RealTagType>,
            pub value: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RealTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::RealTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RealTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RealTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in RealTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in RealTag".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in RealTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RealTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RealTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RealTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for RealTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RealTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RealTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into RealTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// 単一のReal値の定義。allowedValuesとmin・maxは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RealTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::RealTagType,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "min")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<f64>,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "max")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<f64>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::RealTagValue>>,

}



impl RealTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::RealTagType, ) -> RealTagDefinition {
        RealTagDefinition {
 key,
 r_type,
 min: None,
 max: None,
 allowed_values: None,
        }
    }
}

/// Converts the RealTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RealTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min.as_ref().map(|min| {
                [
                    "min".to_string(),
                    min.to_string(),
                ].join(",")
            }),


            self.max.as_ref().map(|max| {
                [
                    "max".to_string(),
                    max.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RealTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RealTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::RealTagType>,
            pub min: Vec<f64>,
            pub max: Vec<f64>,
            pub allowed_values: Vec<Vec<models::RealTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RealTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::RealTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "min" => intermediate_rep.min.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "max" => intermediate_rep.max.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in RealTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RealTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RealTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in RealTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in RealTagDefinition".to_string())?,
            min: intermediate_rep.min.into_iter().next(),
            max: intermediate_rep.max.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RealTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RealTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RealTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for RealTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RealTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RealTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into RealTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum RealTagType {
    #[serde(rename = "real")]
    Real,
}

impl validator::Validate for RealTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for RealTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RealTagType::Real => write!(f, "real"),
        }
    }
}

impl std::str::FromStr for RealTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "real" => std::result::Result::Ok(RealTagType::Real),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RealTagValue(pub f64);

impl validator::Validate for RealTagValue {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<f64> for RealTagValue {
    fn from(x: f64) -> Self {
        RealTagValue(x)
    }
}

impl std::convert::From<RealTagValue> for f64 {
    fn from(x: RealTagValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for RealTagValue {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}

impl std::ops::DerefMut for RealTagValue {
    fn deref_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum SearchTerm {
    MediaTypeMatchTerm(models::MediaTypeMatchTerm),
    TagExistsTerm(models::TagExistsTerm),
    TagMatchTerm(models::TagMatchTerm),
}

impl validator::Validate for SearchTerm
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::MediaTypeMatchTerm(v) => v.validate(),
            Self::TagExistsTerm(v) => v.validate(),
            Self::TagMatchTerm(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SearchTerm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SearchTerm {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::MediaTypeMatchTerm> for SearchTerm {
    fn from(value: models::MediaTypeMatchTerm) -> Self {
        Self::MediaTypeMatchTerm(value)
    }
}
impl From<models::TagExistsTerm> for SearchTerm {
    fn from(value: models::TagExistsTerm) -> Self {
        Self::TagExistsTerm(value)
    }
}
impl From<models::TagMatchTerm> for SearchTerm {
    fn from(value: models::TagMatchTerm) -> Self {
        Self::TagMatchTerm(value)
    }
}





/// コンテンツに付与されたタグ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum Tag {
    KeyOnlyTag(models::KeyOnlyTag),
    TextTag(models::TextTag),
    IntegerTag(models::IntegerTag),
    RealTag(models::RealTag),
    TextSetTag(models::TextSetTag),
    IntegerSetTag(models::IntegerSetTag),
    RealSetTag(models::RealSetTag),
}

impl validator::Validate for Tag
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::KeyOnlyTag(v) => v.validate(),
            Self::TextTag(v) => v.validate(),
            Self::IntegerTag(v) => v.validate(),
            Self::RealTag(v) => v.validate(),
            Self::TextSetTag(v) => v.validate(),
            Self::IntegerSetTag(v) => v.validate(),
            Self::RealSetTag(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Tag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Tag {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::KeyOnlyTag> for Tag {
    fn from(value: models::KeyOnlyTag) -> Self {
        Self::KeyOnlyTag(value)
    }
}
impl From<models::TextTag> for Tag {
    fn from(value: models::TextTag) -> Self {
        Self::TextTag(value)
    }
}
impl From<models::IntegerTag> for Tag {
    fn from(value: models::IntegerTag) -> Self {
        Self::IntegerTag(value)
    }
}
impl From<models::RealTag> for Tag {
    fn from(value: models::RealTag) -> Self {
        Self::RealTag(value)
    }
}
impl From<models::TextSetTag> for Tag {
    fn from(value: models::TextSetTag) -> Self {
        Self::TextSetTag(value)
    }
}
impl From<models::IntegerSetTag> for Tag {
    fn from(value: models::IntegerSetTag) -> Self {
        Self::IntegerSetTag(value)
    }
}
impl From<models::RealSetTag> for Tag {
    fn from(value: models::RealSetTag) -> Self {
        Self::RealSetTag(value)
    }
}





/// タグスキーマに記載されたタグ定義
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[allow(non_camel_case_types, clippy::large_enum_variant)]
pub enum TagDefinition {
    KeyOnlyTagDefinition(models::KeyOnlyTagDefinition),
    TextTagDefinition(models::TextTagDefinition),
    IntegerTagDefinition(models::IntegerTagDefinition),
    RealTagDefinition(models::RealTagDefinition),
    TextSetTagDefinition(models::TextSetTagDefinition),
    IntegerSetTagDefinition(models::IntegerSetTagDefinition),
    RealSetTagDefinition(models::RealSetTagDefinition),
}

impl validator::Validate for TagDefinition
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        match self {
            Self::KeyOnlyTagDefinition(v) => v.validate(),
            Self::TextTagDefinition(v) => v.validate(),
            Self::IntegerTagDefinition(v) => v.validate(),
            Self::RealTagDefinition(v) => v.validate(),
            Self::TextSetTagDefinition(v) => v.validate(),
            Self::IntegerSetTagDefinition(v) => v.validate(),
            Self::RealSetTagDefinition(v) => v.validate(),
        }
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TagDefinition {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}


impl From<models::KeyOnlyTagDefinition> for TagDefinition {
    fn from(value: models::KeyOnlyTagDefinition) -> Self {
        Self::KeyOnlyTagDefinition(value)
    }
}
impl From<models::TextTagDefinition> for TagDefinition {
    fn from(value: models::TextTagDefinition) -> Self {
        Self::TextTagDefinition(value)
    }
}
impl From<models::IntegerTagDefinition> for TagDefinition {
    fn from(value: models::IntegerTagDefinition) -> Self {
        Self::IntegerTagDefinition(value)
    }
}
impl From<models::RealTagDefinition> for TagDefinition {
    fn from(value: models::RealTagDefinition) -> Self {
        Self::RealTagDefinition(value)
    }
}
impl From<models::TextSetTagDefinition> for TagDefinition {
    fn from(value: models::TextSetTagDefinition) -> Self {
        Self::TextSetTagDefinition(value)
    }
}
impl From<models::IntegerSetTagDefinition> for TagDefinition {
    fn from(value: models::IntegerSetTagDefinition) -> Self {
        Self::IntegerSetTagDefinition(value)
    }
}
impl From<models::RealSetTagDefinition> for TagDefinition {
    fn from(value: models::RealSetTagDefinition) -> Self {
        Self::RealSetTagDefinition(value)
    }
}





/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TagExistsKind {
    #[serde(rename = "tagExists")]
    TagExists,
}

impl validator::Validate for TagExistsKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TagExistsKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TagExistsKind::TagExists => write!(f, "tagExists"),
        }
    }
}

impl std::str::FromStr for TagExistsKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "tagExists" => std::result::Result::Ok(TagExistsKind::TagExists),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 指定した名前を持つ、有効なフォーマットかつタグスキーマに整合したタグが型を問わず存在する場合に一致する。 keyのみのタグも対象とするが無効なフォーマットやスキーマ違反のタグへは判定を行わない。 指定したkeyが無効なフォーマットであったりタグスキーマ違反であった場合にはエラーとなる。 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagExistsTerm {
    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::TagExistsKind,

    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

}



impl TagExistsTerm {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(kind: models::TagExistsKind, key: String, ) -> TagExistsTerm {
        TagExistsTerm {
 kind,
 key,
        }
    }
}

/// Converts the TagExistsTerm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TagExistsTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping kind in query parameter serialization


            Some("key".to_string()),
            Some(self.key.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TagExistsTerm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TagExistsTerm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub kind: Vec<models::TagExistsKind>,
            pub key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TagExistsTerm".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::TagExistsKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TagExistsTerm".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TagExistsTerm {
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in TagExistsTerm".to_string())?,
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TagExistsTerm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TagExistsTerm> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TagExistsTerm>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TagExistsTerm>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TagExistsTerm - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TagExistsTerm> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TagExistsTerm as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TagExistsTerm - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagKey(pub String);

impl validator::Validate for TagKey {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for TagKey {
    fn from(x: String) -> Self {
        TagKey(x)
    }
}

impl std::fmt::Display for TagKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TagKey {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TagKey(x.to_string()))
    }
}

impl std::convert::From<TagKey> for String {
    fn from(x: TagKey) -> Self {
        x.0
    }
}

impl std::ops::Deref for TagKey {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TagKey {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TagMatchKind {
    #[serde(rename = "tagMatch")]
    TagMatch,
}

impl validator::Validate for TagMatchKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TagMatchKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TagMatchKind::TagMatch => write!(f, "tagMatch"),
        }
    }
}

impl std::str::FromStr for TagMatchKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "tagMatch" => std::result::Result::Ok(TagMatchKind::TagMatch),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 指定したkeyを持つタグの値が、指定値のいずれかと一致する場合に一致する。 指定値は文字列で受け取り、そのkeyにタグスキーマの定義があれば定義された型と制約に従って解釈する。 定義がなければ、keyが許容される場合にTextとして解釈する。値の字句からIntegerやRealを推測しない。 セット型では定義された要素型で解釈し、いずれかの要素が指定値のいずれかと一致すれば一致する。 タグが存在しない場合や、定義のないkeyのみのタグに対しては一致しない。 keyのみと定義されたタグへの値一致、フォーマットが無効なkeyや値、タグスキーマに違反するkeyや値を指定した場合にはエラーとなる。 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagMatchTerm {
    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::TagMatchKind,

    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    /// 各値をタグスキーマの定義またはTextのフォールバックに従って解釈する。空文字列は指定できない。
    #[serde(rename = "values")]
    #[validate(
            length(min = 1),
          custom(function = "check_xss_vec_string"),
    )]
    pub values: Vec<String>,

}



impl TagMatchTerm {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(kind: models::TagMatchKind, key: String, values: Vec<String>, ) -> TagMatchTerm {
        TagMatchTerm {
 kind,
 key,
 values,
        }
    }
}

/// Converts the TagMatchTerm value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TagMatchTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping kind in query parameter serialization


            Some("key".to_string()),
            Some(self.key.to_string()),


            Some("values".to_string()),
            Some(self.values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TagMatchTerm value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TagMatchTerm {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub kind: Vec<models::TagMatchKind>,
            pub key: Vec<String>,
            pub values: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TagMatchTerm".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::TagMatchKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in TagMatchTerm".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TagMatchTerm".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TagMatchTerm {
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in TagMatchTerm".to_string())?,
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TagMatchTerm".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in TagMatchTerm".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TagMatchTerm> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TagMatchTerm>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TagMatchTerm>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TagMatchTerm - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TagMatchTerm> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TagMatchTerm as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TagMatchTerm - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// コンテンツの登録・検索・診断に適用する現在有効なタグスキーマ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TagSchema {
    #[serde(rename = "version")]
          #[validate(nested)]
    pub version: models::TagSchemaVersion,

    /// 定義されていないキーのタグを許可するか
    #[serde(rename = "allowAdditionalTags")]
    pub allow_additional_tags: bool,

    /// 必須タグの定義。定義がない場合は空配列
    #[serde(rename = "required")]
          #[validate(nested)]
    pub required: Vec<models::TagDefinition>,

    /// 任意タグの定義。定義がない場合は空配列
    #[serde(rename = "optional")]
          #[validate(nested)]
    pub optional: Vec<models::TagDefinition>,

}



impl TagSchema {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(version: models::TagSchemaVersion, allow_additional_tags: bool, required: Vec<models::TagDefinition>, optional: Vec<models::TagDefinition>, ) -> TagSchema {
        TagSchema {
 version,
 allow_additional_tags,
 required,
 optional,
        }
    }
}

/// Converts the TagSchema value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TagSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping version in query parameter serialization


            Some("allowAdditionalTags".to_string()),
            Some(self.allow_additional_tags.to_string()),

            // Skipping required in query parameter serialization

            // Skipping optional in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TagSchema value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TagSchema {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub version: Vec<models::TagSchemaVersion>,
            pub allow_additional_tags: Vec<bool>,
            pub required: Vec<Vec<models::TagDefinition>>,
            pub optional: Vec<Vec<models::TagDefinition>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TagSchema".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<models::TagSchemaVersion as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "allowAdditionalTags" => intermediate_rep.allow_additional_tags.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "required" => return std::result::Result::Err("Parsing a container in this style is not supported in TagSchema".to_string()),
                    "optional" => return std::result::Result::Err("Parsing a container in this style is not supported in TagSchema".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TagSchema".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TagSchema {
            version: intermediate_rep.version.into_iter().next().ok_or_else(|| "version missing in TagSchema".to_string())?,
            allow_additional_tags: intermediate_rep.allow_additional_tags.into_iter().next().ok_or_else(|| "allowAdditionalTags missing in TagSchema".to_string())?,
            required: intermediate_rep.required.into_iter().next().ok_or_else(|| "required missing in TagSchema".to_string())?,
            optional: intermediate_rep.optional.into_iter().next().ok_or_else(|| "optional missing in TagSchema".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TagSchema> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TagSchema>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TagSchema>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TagSchema - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TagSchema> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TagSchema as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TagSchema - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// タグスキーマの形式バージョン
/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TagSchemaVersion {
    #[serde(rename = "0")]
    Variant0,
}

impl validator::Validate for TagSchemaVersion
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TagSchemaVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TagSchemaVersion::Variant0 => write!(f, "0"),
        }
    }
}

impl std::str::FromStr for TagSchemaVersion {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => std::result::Result::Ok(TagSchemaVersion::Variant0),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// Text値からなる集合のタグ。要素は順序を持たず、値の重複は許可しない。JSONでは配列として表現する
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextSetTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::TextSetTagType,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::TextTagValue>,

}



impl TextSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::TextSetTagType, values: Vec<models::TextTagValue>, ) -> TextSetTag {
        TextSetTag {
 key,
 r_type,
 values,
        }
    }
}

/// Converts the TextSetTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TextSetTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("values".to_string()),
            Some(self.values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextSetTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextSetTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::TextSetTagType>,
            pub values: Vec<Vec<models::TextTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextSetTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::TextSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in TextSetTag".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextSetTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextSetTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TextSetTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in TextSetTag".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in TextSetTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextSetTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TextSetTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextSetTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TextSetTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TextSetTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextSetTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TextSetTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Text値からなる集合の定義。allowedValuesとminLength・maxLengthは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextSetTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::TextSetTagType,

    #[serde(rename = "minLength")]
    #[validate(
            range(min = 1u32, max = 255u32),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_length: Option<u32>,

    #[serde(rename = "maxLength")]
    #[validate(
            range(min = 1u32, max = 255u32),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_length: Option<u32>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::TextTagValue>>,

}



impl TextSetTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::TextSetTagType, ) -> TextSetTagDefinition {
        TextSetTagDefinition {
 key,
 r_type,
 min_length: None,
 max_length: None,
 allowed_values: None,
        }
    }
}

/// Converts the TextSetTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TextSetTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min_length.as_ref().map(|min_length| {
                [
                    "minLength".to_string(),
                    min_length.to_string(),
                ].join(",")
            }),


            self.max_length.as_ref().map(|max_length| {
                [
                    "maxLength".to_string(),
                    max_length.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextSetTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextSetTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::TextSetTagType>,
            pub min_length: Vec<u32>,
            pub max_length: Vec<u32>,
            pub allowed_values: Vec<Vec<models::TextTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextSetTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::TextSetTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "minLength" => intermediate_rep.min_length.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxLength" => intermediate_rep.max_length.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in TextSetTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextSetTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextSetTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TextSetTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in TextSetTagDefinition".to_string())?,
            min_length: intermediate_rep.min_length.into_iter().next(),
            max_length: intermediate_rep.max_length.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextSetTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TextSetTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextSetTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TextSetTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TextSetTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextSetTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TextSetTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TextSetTagType {
    #[serde(rename = "textSet")]
    TextSet,
}

impl validator::Validate for TextSetTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TextSetTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextSetTagType::TextSet => write!(f, "textSet"),
        }
    }
}

impl std::str::FromStr for TextSetTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "textSet" => std::result::Result::Ok(TextSetTagType::TextSet),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 単一のText値を持つタグ
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextTag {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::TextTagType,

    /// 長さが1以上255以下の有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFCで正規化済みの値。長さはUnicodeコードポイント数で数える
    #[serde(rename = "value")]
    #[validate(
            length(min = 1, max = 255),
          custom(function = "check_xss_string"),
    )]
    pub value: String,

}



impl TextTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::TextTagType, value: String, ) -> TextTag {
        TextTag {
 key,
 r_type,
 value,
        }
    }
}

/// Converts the TextTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TextTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::TextTagType>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::TextTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TextTag".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in TextTag".to_string())?,
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in TextTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TextTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TextTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TextTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TextTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// 単一のText値の定義。allowedValuesとminLength・maxLengthは同時に指定しない
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextTagDefinition {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "type")]
          #[validate(nested)]
    pub r_type: models::TextTagType,

    #[serde(rename = "minLength")]
    #[validate(
            range(min = 1u32, max = 255u32),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_length: Option<u32>,

    #[serde(rename = "maxLength")]
    #[validate(
            range(min = 1u32, max = 255u32),
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_length: Option<u32>,

    #[serde(rename = "allowedValues")]
    #[validate(
          nested,
    )]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allowed_values: Option<Vec<models::TextTagValue>>,

}



impl TextTagDefinition {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: models::TextTagType, ) -> TextTagDefinition {
        TextTagDefinition {
 key,
 r_type,
 min_length: None,
 max_length: None,
 allowed_values: None,
        }
    }
}

/// Converts the TextTagDefinition value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TextTagDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping type in query parameter serialization


            self.min_length.as_ref().map(|min_length| {
                [
                    "minLength".to_string(),
                    min_length.to_string(),
                ].join(",")
            }),


            self.max_length.as_ref().map(|max_length| {
                [
                    "maxLength".to_string(),
                    max_length.to_string(),
                ].join(",")
            }),


            self.allowed_values.as_ref().map(|allowed_values| {
                [
                    "allowedValues".to_string(),
                    allowed_values.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TextTagDefinition value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TextTagDefinition {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub r_type: Vec<models::TextTagType>,
            pub min_length: Vec<u32>,
            pub max_length: Vec<u32>,
            pub allowed_values: Vec<Vec<models::TextTagValue>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TextTagDefinition".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<models::TextTagType as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "minLength" => intermediate_rep.min_length.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxLength" => intermediate_rep.max_length.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "allowedValues" => return std::result::Result::Err("Parsing a container in this style is not supported in TextTagDefinition".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing TextTagDefinition".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TextTagDefinition {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in TextTagDefinition".to_string())?,
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in TextTagDefinition".to_string())?,
            min_length: intermediate_rep.min_length.into_iter().next(),
            max_length: intermediate_rep.max_length.into_iter().next(),
            allowed_values: intermediate_rep.allowed_values.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TextTagDefinition> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TextTagDefinition>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TextTagDefinition>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for TextTagDefinition - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TextTagDefinition> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TextTagDefinition as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into TextTagDefinition - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum TextTagType {
    #[serde(rename = "text")]
    Text,
}

impl validator::Validate for TextTagType
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for TextTagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            TextTagType::Text => write!(f, "text"),
        }
    }
}

impl std::str::FromStr for TextTagType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "text" => std::result::Result::Ok(TextTagType::Text),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 長さが1以上255以下の有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFCで正規化済みの値。長さはUnicodeコードポイント数で数える
#[derive(Debug, Clone, PartialEq, PartialOrd,  serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TextTagValue(pub String);

impl validator::Validate for TextTagValue {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {

        std::result::Result::Ok(())
    }
}

impl std::convert::From<String> for TextTagValue {
    fn from(x: String) -> Self {
        TextTagValue(x)
    }
}

impl std::fmt::Display for TextTagValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for TextTagValue {
    type Err = std::string::ParseError;
    fn from_str(x: &str) -> std::result::Result<Self, Self::Err> {
        std::result::Result::Ok(TextTagValue(x.to_string()))
    }
}

impl std::convert::From<TextTagValue> for String {
    fn from(x: TextTagValue) -> Self {
        x.0
    }
}

impl std::ops::Deref for TextTagValue {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

impl std::ops::DerefMut for TextTagValue {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnparseableTagValueDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::UnparseableTagValueDiagnosticKind,

    #[serde(rename = "definition")]
          #[validate(nested)]
    #[serde(skip_serializing_if="Option::is_none")]
    pub definition: Option<models::TagDefinition>,

}



impl UnparseableTagValueDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::UnparseableTagValueDiagnosticKind, ) -> UnparseableTagValueDiagnostic {
        UnparseableTagValueDiagnostic {
 key,
 kind,
 definition: None,
        }
    }
}

/// Converts the UnparseableTagValueDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnparseableTagValueDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

            // Skipping definition in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnparseableTagValueDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnparseableTagValueDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::UnparseableTagValueDiagnosticKind>,
            pub definition: Vec<models::TagDefinition>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UnparseableTagValueDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::UnparseableTagValueDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "definition" => intermediate_rep.definition.push(<models::TagDefinition as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnparseableTagValueDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnparseableTagValueDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in UnparseableTagValueDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in UnparseableTagValueDiagnostic".to_string())?,
            definition: intermediate_rep.definition.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnparseableTagValueDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnparseableTagValueDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnparseableTagValueDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UnparseableTagValueDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnparseableTagValueDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnparseableTagValueDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UnparseableTagValueDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum UnparseableTagValueDiagnosticKind {
    #[serde(rename = "unparseableTagValue")]
    UnparseableTagValue,
}

impl validator::Validate for UnparseableTagValueDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for UnparseableTagValueDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnparseableTagValueDiagnosticKind::UnparseableTagValue => write!(f, "unparseableTagValue"),
        }
    }
}

impl std::str::FromStr for UnparseableTagValueDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "unparseableTagValue" => std::result::Result::Ok(UnparseableTagValueDiagnosticKind::UnparseableTagValue),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


/// 対応していないメディアタイプを表すProblem Details
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnsupportedMediaTypeProblem {
    /// エラー種別を識別するURI参照
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// エラー種別の短い説明
    #[serde(rename = "title")]
          #[validate(custom(function = "check_xss_string"))]
    pub title: String,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "status")]
    pub status: i32,

    /// このエラーの具体的な説明
    #[serde(rename = "detail")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<String>,

    /// このエラー発生を識別するURI参照
    #[serde(rename = "instance")]
          #[validate(custom(function = "check_xss_string"))]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,

}



impl UnsupportedMediaTypeProblem {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(title: String, status: i32, ) -> UnsupportedMediaTypeProblem {
        UnsupportedMediaTypeProblem {
 r_type: r#"about:blank"#.to_string(),
 title,
 status,
 detail: None,
 instance: None,
        }
    }
}

/// Converts the UnsupportedMediaTypeProblem value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnsupportedMediaTypeProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r_type.to_string()),


            Some("title".to_string()),
            Some(self.title.to_string()),


            Some("status".to_string()),
            Some(self.status.to_string()),


            self.detail.as_ref().map(|detail| {
                [
                    "detail".to_string(),
                    detail.to_string(),
                ].join(",")
            }),


            self.instance.as_ref().map(|instance| {
                [
                    "instance".to_string(),
                    instance.to_string(),
                ].join(",")
            }),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnsupportedMediaTypeProblem value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnsupportedMediaTypeProblem {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r_type: Vec<String>,
            pub title: Vec<String>,
            pub status: Vec<i32>,
            pub detail: Vec<String>,
            pub instance: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UnsupportedMediaTypeProblem".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "title" => intermediate_rep.title.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "detail" => intermediate_rep.detail.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "instance" => intermediate_rep.instance.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnsupportedMediaTypeProblem".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnsupportedMediaTypeProblem {
            r_type: intermediate_rep.r_type.into_iter().next().ok_or_else(|| "type missing in UnsupportedMediaTypeProblem".to_string())?,
            title: intermediate_rep.title.into_iter().next().ok_or_else(|| "title missing in UnsupportedMediaTypeProblem".to_string())?,
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in UnsupportedMediaTypeProblem".to_string())?,
            detail: intermediate_rep.detail.into_iter().next(),
            instance: intermediate_rep.instance.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnsupportedMediaTypeProblem> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnsupportedMediaTypeProblem>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnsupportedMediaTypeProblem>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UnsupportedMediaTypeProblem - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnsupportedMediaTypeProblem> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnsupportedMediaTypeProblem as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UnsupportedMediaTypeProblem - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UnsupportedXmpValueTypeDiagnostic {
    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFKCで正規化済みのタグ名。 先頭はUnicode UAX #31のXID_Start、残りはXID_Continueに属する必要がある。 長さはUnicodeコードポイント数で数える。 
    #[serde(rename = "key")]
    #[validate(
            length(min = 1, max = 64),
          custom(function = "check_xss_string"),
    )]
    pub key: String,

    #[serde(rename = "kind")]
          #[validate(nested)]
    pub kind: models::UnsupportedXmpValueTypeDiagnosticKind,

}



impl UnsupportedXmpValueTypeDiagnostic {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, kind: models::UnsupportedXmpValueTypeDiagnosticKind, ) -> UnsupportedXmpValueTypeDiagnostic {
        UnsupportedXmpValueTypeDiagnostic {
 key,
 kind,
        }
    }
}

/// Converts the UnsupportedXmpValueTypeDiagnostic value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for UnsupportedXmpValueTypeDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

            // Skipping kind in query parameter serialization

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a UnsupportedXmpValueTypeDiagnostic value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for UnsupportedXmpValueTypeDiagnostic {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub kind: Vec<models::UnsupportedXmpValueTypeDiagnosticKind>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing UnsupportedXmpValueTypeDiagnostic".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kind" => intermediate_rep.kind.push(<models::UnsupportedXmpValueTypeDiagnosticKind as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing UnsupportedXmpValueTypeDiagnostic".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(UnsupportedXmpValueTypeDiagnostic {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in UnsupportedXmpValueTypeDiagnostic".to_string())?,
            kind: intermediate_rep.kind.into_iter().next().ok_or_else(|| "kind missing in UnsupportedXmpValueTypeDiagnostic".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<UnsupportedXmpValueTypeDiagnostic> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<UnsupportedXmpValueTypeDiagnostic>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<UnsupportedXmpValueTypeDiagnostic>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for UnsupportedXmpValueTypeDiagnostic - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<UnsupportedXmpValueTypeDiagnostic> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <UnsupportedXmpValueTypeDiagnostic as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into UnsupportedXmpValueTypeDiagnostic - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// Enumeration of values.
/// Since this enum's variants do not hold data, we can easily define them as `#[repr(C)]`
/// which helps with FFI.
#[allow(non_camel_case_types, clippy::large_enum_variant)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk_enum_derive::LabelledGenericEnum))]
pub enum UnsupportedXmpValueTypeDiagnosticKind {
    #[serde(rename = "unsupportedXmpValueType")]
    UnsupportedXmpValueType,
}

impl validator::Validate for UnsupportedXmpValueTypeDiagnosticKind
{
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        std::result::Result::Ok(())
    }
}

impl std::fmt::Display for UnsupportedXmpValueTypeDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            UnsupportedXmpValueTypeDiagnosticKind::UnsupportedXmpValueType => write!(f, "unsupportedXmpValueType"),
        }
    }
}

impl std::str::FromStr for UnsupportedXmpValueTypeDiagnosticKind {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "unsupportedXmpValueType" => std::result::Result::Ok(UnsupportedXmpValueTypeDiagnosticKind::UnsupportedXmpValueType),
            _ => std::result::Result::Err(format!(r#"Value not valid: {s}"#)),
        }
    }
}


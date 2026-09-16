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

    /// 有効なタグ。該当するタグがなければ空配列を返す。 同一コンテンツ内のタグのkeyは一意とし、値や型が異なる場合も同じkeyを持つタグの重複は認めない。 
    #[serde(rename = "tags")]
          #[validate(nested)]
    pub tags: Vec<models::Tag>,

    /// 無効なタグ。無効となった簡単な理由を含む。該当するタグがなければ空配列を返す
    #[serde(rename = "invalidTags")]
          #[validate(nested)]
    pub invalid_tags: Vec<models::InvalidTag>,

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
    pub fn new(id: String, media_type: models::MediaType, content_url: String, thumbnail_url: String, tags: Vec<models::Tag>, invalid_tags: Vec<models::InvalidTag>, ) -> Content {
        Content {
 id,
 media_type,
 content_url,
 thumbnail_url,
 tags,
 invalid_tags,
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

            // Skipping invalidTags in query parameter serialization

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
            pub invalid_tags: Vec<Vec<models::InvalidTag>>,
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
                    "invalidTags" => return std::result::Result::Err("Parsing a container in this style is not supported in Content".to_string()),
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
            invalid_tags: intermediate_rep.invalid_tags.into_iter().next().ok_or_else(|| "invalidTags missing in Content".to_string())?,
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::IntegerTagValue>,

}



impl IntegerSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, values: Vec<models::IntegerTagValue>, ) -> IntegerSetTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// JavaScriptのnumberで安全に扱える整数範囲（-(2^53 - 1)以上、2^53 - 1以下）に限定されたJSON数値 
    #[serde(rename = "value")]
    #[validate(
            range(min = -9007199254740991i64, max = 9007199254740991i64),
    )]
    pub value: i64,

}



impl IntegerTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, value: i64, ) -> IntegerTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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



/// 無効なタグの情報 
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct InvalidTag {
    /// 無効となる該当タグ名
    #[serde(rename = "key")]
          #[validate(custom(function = "check_xss_string"))]
    pub key: String,

    /// - `INVALID_KEY`: タグ名が条件を満たしていない - `UNSUPPORTED_VALUE_TYPE`: タグの型がサポートされていない - `INVALID_VALUE`: 値が条件を満たしていない 
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "reason")]
          #[validate(custom(function = "check_xss_string"))]
    pub reason: String,

}



impl InvalidTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, reason: String, ) -> InvalidTag {
        InvalidTag {
 key,
 reason,
        }
    }
}

/// Converts the InvalidTag value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for InvalidTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),


            Some("reason".to_string()),
            Some(self.reason.to_string()),

        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a InvalidTag value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for InvalidTag {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub reason: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing InvalidTag".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reason" => intermediate_rep.reason.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing InvalidTag".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(InvalidTag {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in InvalidTag".to_string())?,
            reason: intermediate_rep.reason.into_iter().next().ok_or_else(|| "reason missing in InvalidTag".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<InvalidTag> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<InvalidTag>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<InvalidTag>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Invalid header value for InvalidTag - value: {hdr_value} is invalid {e}"#))
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<InvalidTag> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <InvalidTag as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(format!(r#"Unable to convert header value '{value}' into InvalidTag - {err}"#))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(format!(r#"Unable to convert header: {hdr_value:?} to string: {e}"#))
        }
    }
}



/// キーのみのタグ
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

}



impl KeyOnlyTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, ) -> KeyOnlyTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),

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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::RealTagValue>,

}



impl RealSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, values: Vec<models::RealTagValue>, ) -> RealSetTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// XMPのRealの字句形式に従い、IEEE 754 binary64の有限値として変換可能な値をJSON数値で返す
    #[serde(rename = "value")]
    pub value: f64,

}



impl RealTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, value: f64, ) -> RealTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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


/// 指定した名前の有効なタグが型を問わず存在する場合に一致する。キーのみのタグも対象とするがinvalidTagsは対象外。
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


/// 指定したキーを持つタグの値が、指定値のいずれかと一致する場合に一致する。 タグが集合型の場合には、いずれかの要素が指定値のいずれかと一致すれば一致する。 有効な文字列型・文字列集合型のタグが存在しない場合は不一致とする。 
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

    #[serde(rename = "values")]
    #[validate(
            length(min = 1),
          nested,
    )]
    pub values: Vec<models::TextTagValue>,

}



impl TagMatchTerm {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(kind: models::TagMatchKind, key: String, values: Vec<models::TextTagValue>, ) -> TagMatchTerm {
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
            pub values: Vec<Vec<models::TextTagValue>>,
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    #[serde(rename = "values")]
    #[validate(
          nested,
    )]
    pub values: Vec<models::TextTagValue>,

}



impl TextSetTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, values: Vec<models::TextTagValue>, ) -> TextSetTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
          #[validate(custom(function = "check_xss_string"))]
    pub r_type: String,

    /// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFCで正規化済みの値
    #[serde(rename = "value")]
          #[validate(custom(function = "check_xss_string"))]
    pub value: String,

}



impl TextTag {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(key: String, r_type: String, value: String, ) -> TextTag {
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


            Some("type".to_string()),
            Some(self.r_type.to_string()),


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
            pub r_type: Vec<String>,
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
                    "type" => intermediate_rep.r_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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



/// 有効なUTF-8文字列で、デコード後のUnicodeコードポイント列がNFCで正規化済みの値
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



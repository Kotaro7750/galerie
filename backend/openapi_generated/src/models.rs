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

    /// コンテンツファイルのメディアタイプ
    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "mediaType")]
          #[validate(custom(function = "check_xss_string"))]
    pub media_type: String,

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
    pub fn new(id: String, media_type: String, content_url: String, thumbnail_url: String, ) -> Content {
        Content {
 id,
 media_type,
 content_url,
 thumbnail_url,
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


            Some("mediaType".to_string()),
            Some(self.media_type.to_string()),


            Some("contentUrl".to_string()),
            Some(self.content_url.to_string()),


            Some("thumbnailUrl".to_string()),
            Some(self.thumbnail_url.to_string()),

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
            pub media_type: Vec<String>,
            pub content_url: Vec<String>,
            pub thumbnail_url: Vec<String>,
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
                    "mediaType" => intermediate_rep.media_type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "contentUrl" => intermediate_rep.content_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "thumbnailUrl" => intermediate_rep.thumbnail_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
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



/// コンテンツが存在しないことを表すProblem Details
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



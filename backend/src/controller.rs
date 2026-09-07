use axum::Json;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use mime::Mime;

use galerie_api::models::{
    BadRequestProblem, Content, ContentNotFoundProblem, ContentPage, GetContentPathParams,
    IntegerSetTag, IntegerTag, IntegerTagValue, InternalServerErrorProblem, InvalidTag, KeyOnlyTag,
    ListContentsQueryParams, RealSetTag, RealTag, RealTagValue, Tag, TextSetTag, TextTag,
    TextTagValue,
};

use crate::domain::tag::{SkippedTag, Tag as DomainTag};
use crate::usecase::{GetContentUseCase, ListContentsUseCase};
use crate::{domain, usecase};

#[derive(Clone)]
pub(crate) struct ContentController {
    list_contents: ListContentsUseCase,
    get_content: GetContentUseCase,
}

impl ContentController {
    pub(crate) fn new(
        list_contents: usecase::ListContentsUseCase,
        get_content: usecase::GetContentUseCase,
    ) -> Self {
        ContentController {
            list_contents,
            get_content,
        }
    }

    pub(crate) fn router(self) -> axum::Router {
        axum::Router::new()
            .route("/{content_id}", axum::routing::get(Self::get_content))
            .route("/", axum::routing::get(Self::list_contents))
            .with_state(self)
    }

    pub(crate) async fn get_content(
        State(this): State<Self>,
        path: Result<Path<GetContentPathParams>, PathRejection>,
    ) -> Result<Json<Content>, ContentControlllerError> {
        let id = path?
            .content_id
            .parse::<domain::ContentId>()
            .map_err(|e| {
                ContentControlllerError::BadRequest(format!("invalid contentId, err: {}", e))
            })?;

        let content = this.get_content.execute(id)?;

        Ok(Json(content.into()))
    }

    pub(crate) async fn list_contents(
        State(this): State<Self>,
        query: Result<Query<ListContentsQueryParams>, QueryRejection>,
    ) -> Result<Json<ContentPage>, ContentControlllerError> {
        let query_params = query?;

        let limit = query_params.limit.unwrap_or(30);
        if limit <= 0 || 100 < limit {
            return Err(ContentControlllerError::BadRequest(
                "limit must be between 1 and 100".to_string(),
            ));
        }

        if let Some(cursor) = &query_params.cursor {
            if cursor.is_empty() {
                return Err(ContentControlllerError::BadRequest(
                    "cursor must not be empty".to_string(),
                ));
            }
        }

        let (items, cursor) = this
            .list_contents
            .execute(limit.into(), query_params.cursor.clone())?;

        Ok(Json(ContentPage {
            items: items.into_iter().map(|c| c.into()).collect(),
            next_cursor: cursor,
        }))
    }
}

impl From<domain::Content> for Content {
    fn from(content: domain::Content) -> Self {
        Content {
            id: content.id().as_ref().to_string(),
            media_type: Into::<Mime>::into(content.media_type()).to_string(),
            content_url: content.content_url().to_string(),
            thumbnail_url: content.thumbnail_url().to_string(),
            tags: content
                .tags()
                .iter()
                .map(|tag| tag.clone().into())
                .collect(),
            invalid_tags: content
                .skipped_tags()
                .iter()
                .map(|tag| tag.clone().into())
                .collect(),
        }
    }
}

impl From<DomainTag> for Tag {
    fn from(tag: DomainTag) -> Self {
        match tag {
            DomainTag::KeyOnly { key } => Self::KeyOnlyTag(KeyOnlyTag {
                key: key.as_ref().to_string(),
                r_type: "keyOnly".to_string(),
            }),
            DomainTag::Text { key, value } => Self::TextTag(TextTag {
                key: key.as_ref().to_string(),
                value: value.as_ref().to_string(),
                r_type: "text".to_string(),
            }),
            DomainTag::Integer { key, value } => Self::IntegerTag(IntegerTag {
                key: key.as_ref().to_string(),
                value: *value.as_ref(),
                r_type: "integer".to_string(),
            }),
            DomainTag::Real { key, value } => Self::RealTag(RealTag {
                key: key.as_ref().to_string(),
                value: *value.as_ref(),
                r_type: "real".to_string(),
            }),
            DomainTag::TextSet { key, values } => Self::TextSetTag(TextSetTag {
                key: key.as_ref().to_string(),
                values: values
                    .into_iter()
                    .map(|v| TextTagValue(v.as_ref().to_string()))
                    .collect(),
                r_type: "textSet".to_string(),
            }),
            DomainTag::IntegerSet { key, values } => Self::IntegerSetTag(IntegerSetTag {
                key: key.as_ref().to_string(),
                values: values
                    .into_iter()
                    .map(|v| IntegerTagValue(*v.as_ref()))
                    .collect(),
                r_type: "integerSet".to_string(),
            }),
            DomainTag::RealSet { key, values } => Self::RealSetTag(RealSetTag {
                key: key.as_ref().to_string(),
                values: values
                    .into_iter()
                    .map(|v| RealTagValue(*v.as_ref()))
                    .collect(),
                r_type: "realSet".to_string(),
            }),
        }
    }
}

impl From<SkippedTag> for InvalidTag {
    fn from(tag: SkippedTag) -> Self {
        InvalidTag {
            key: tag.key().to_string(),
            reason: match tag.reason() {
                crate::domain::tag::SkippedReason::InvalidKey => "INVALID_KEY".to_string(),
                crate::domain::tag::SkippedReason::InvalidValue => "INVALID_VALUE".to_string(),
                crate::domain::tag::SkippedReason::UnsupportedValueType => {
                    "UNSUPPORTED_VALUE_TYPE".to_string()
                }
            },
        }
    }
}

#[derive(Debug)]
pub(crate) enum ContentControlllerError {
    BadRequest(String),
    ContentNotFound,
    InternalServerError(String),
}

impl From<PathRejection> for ContentControlllerError {
    fn from(err: PathRejection) -> Self {
        ContentControlllerError::BadRequest(format!("invalid path parameters, err: {}", err))
    }
}

impl From<QueryRejection> for ContentControlllerError {
    fn from(err: QueryRejection) -> Self {
        ContentControlllerError::BadRequest(format!("invalid query parameters, err: {}", err))
    }
}

impl From<domain::Error> for ContentControlllerError {
    fn from(err: domain::Error) -> Self {
        match err {
            domain::Error::ContentNotFound => Self::ContentNotFound,
            domain::Error::InvalidCursor => Self::BadRequest("invalid cursor".to_string()),
            domain::Error::Internal(e) => Self::InternalServerError(e),
        }
    }
}

impl IntoResponse for ContentControlllerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(e) => (
                StatusCode::BAD_REQUEST,
                [(CONTENT_TYPE, "application/problem+json")],
                Json(BadRequestProblem::new(
                    e,
                    StatusCode::BAD_REQUEST.as_u16().into(),
                )),
            )
                .into_response(),

            Self::ContentNotFound => (
                StatusCode::NOT_FOUND,
                [(CONTENT_TYPE, "application/problem+json")],
                Json(ContentNotFoundProblem::new(
                    "content not found".to_string(),
                    StatusCode::NOT_FOUND.as_u16().into(),
                )),
            )
                .into_response(),

            Self::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(CONTENT_TYPE, "application/problem+json")],
                Json(InternalServerErrorProblem::new(
                    e,
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16().into(),
                )),
            )
                .into_response(),
        }
    }
}

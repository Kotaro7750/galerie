use axum::Json;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use mime::Mime;
use uuid::Uuid;

use galerie_api::models::{
    BadRequestProblem, Content, ContentNotFoundProblem, ContentPage, GetContentPathParams,
    InternalServerErrorProblem, ListContentsQueryParams,
};

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
        let id = Uuid::parse_str(&path?.content_id)
            // Check if passed id is a valid UUID
            .map_err(|e| {
                ContentControlllerError::BadRequest(format!(
                    "cannot parse contentId as valid UUID, err: {}",
                    e
                ))
            })?
            // Check if passed id is castable to ContentId
            .try_into()
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

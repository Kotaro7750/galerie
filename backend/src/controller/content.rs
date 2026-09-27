use axum::Json;
use axum::extract::rejection::{PathRejection, QueryRejection};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use http::StatusCode;
use http::header::CONTENT_TYPE;

use galerie_api::models::{
    BadRequestProblem, Content, ContentNotFoundProblem, ContentPage, GetContentPathParams,
    InternalServerErrorProblem, SearchTerm,
};

use crate::domain::search_condition::{FileFormatPredicate, TagPredicate, Term};
use crate::domain::tag_schema::TagSchema;
use crate::usecase::{GetContentUseCase, ListContentsUseCase};
use crate::{domain, usecase};

#[derive(serde::Deserialize)]
pub(crate) struct ListContentsQueryParams {
    cursor: Option<String>,
    limit: Option<u8>,
    condition: Option<String>,
}

#[derive(Clone)]
pub(crate) struct ContentController {
    tag_schema: TagSchema,
    list_contents: ListContentsUseCase,
    get_content: GetContentUseCase,
}

impl ContentController {
    pub(crate) fn new(
        tag_schema: TagSchema,
        list_contents: usecase::ListContentsUseCase,
        get_content: usecase::GetContentUseCase,
    ) -> Self {
        ContentController {
            tag_schema,
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
    ) -> Result<Json<Content>, ContentControllerError> {
        let id = path?.content_id.parse::<domain::ContentId>().map_err(|e| {
            ContentControllerError::BadRequest(format!("invalid contentId, err: {}", e))
        })?;

        let content = this.get_content.execute(id)?;

        Ok(Json(content.into()))
    }

    pub(crate) async fn list_contents(
        State(this): State<Self>,
        query: Result<Query<ListContentsQueryParams>, QueryRejection>,
    ) -> Result<Json<ContentPage>, ContentControllerError> {
        let query_params = query?;

        let limit = query_params.limit.unwrap_or(30);
        if limit == 0 || 100 < limit {
            return Err(ContentControllerError::BadRequest(
                "limit must be between 1 and 100".to_string(),
            ));
        }

        if let Some(cursor) = &query_params.cursor
            && cursor.is_empty()
        {
            return Err(ContentControllerError::BadRequest(
                "cursor must not be empty".to_string(),
            ));
        }

        let conditions = query_params
            .condition
            .as_deref()
            .map(serde_json::from_str::<Vec<SearchTerm>>)
            .transpose()
            .map_err(|e| {
                ContentControllerError::BadRequest(format!("invalid search condition, err: {}", e))
            })?
            .unwrap_or_default();

        let mut search_terms = vec![];
        for condition in &conditions {
            let term = match condition {
                SearchTerm::MediaTypeMatchTerm(term) => Term::new_file_format(
                    FileFormatPredicate::new_match(
                        term.values
                            .iter()
                            .map(|m| (*m).into())
                            .collect::<Vec<domain::MediaType>>()
                            .as_slice(),
                    )
                    .ok_or(ContentControllerError::BadRequest(
                        "invalid search condition term for media type".to_string(),
                    ))?,
                ),
                SearchTerm::TagExistsTerm(term) => {
                    let key = domain::tag::TagKey::new(term.key.as_str()).ok_or(
                        ContentControllerError::BadRequest(
                            "invalid search condition term for tag existence".to_string(),
                        ),
                    )?;
                    if !this.tag_schema.is_tag_key_allowed(&key) {
                        return Err(ContentControllerError::BadRequest(
                            "invalid search condition term for tag existence".to_string(),
                        ));
                    }
                    Term::new_tag(key, TagPredicate::Exists)
                }
                SearchTerm::TagMatchTerm(term) => {
                    let key = domain::tag::TagKey::new(term.key.as_str()).ok_or(
                        ContentControllerError::BadRequest(
                            "invalid search condition term for tag match".to_string(),
                        ),
                    )?;

                    this.tag_schema
                        .construct_search_term(&key, &term.values)
                        .ok_or(ContentControllerError::BadRequest(
                            "invalid search condition term for tag match".to_string(),
                        ))?
                }
            };

            search_terms.push(term);
        }

        let (items, cursor) = this.list_contents.execute(
            limit.into(),
            query_params.cursor.clone(),
            Some(domain::search_condition::SearchCondition::new(
                search_terms.as_slice(),
            )),
        )?;

        Ok(Json(ContentPage {
            items: items.into_iter().map(|c| c.into()).collect(),
            next_cursor: cursor,
        }))
    }
}

#[derive(Debug)]
pub(crate) enum ContentControllerError {
    BadRequest(String),
    ContentNotFound,
    InternalServerError(String),
}

impl From<PathRejection> for ContentControllerError {
    fn from(err: PathRejection) -> Self {
        ContentControllerError::BadRequest(format!("invalid path parameters, err: {}", err))
    }
}

impl From<QueryRejection> for ContentControllerError {
    fn from(err: QueryRejection) -> Self {
        ContentControllerError::BadRequest(format!("invalid query parameters, err: {}", err))
    }
}

impl From<domain::Error> for ContentControllerError {
    fn from(err: domain::Error) -> Self {
        match err {
            domain::Error::ContentNotFound => Self::ContentNotFound,
            domain::Error::InvalidCursor => Self::BadRequest("invalid cursor".to_string()),
            domain::Error::Internal(e) => Self::InternalServerError(e),
        }
    }
}

impl IntoResponse for ContentControllerError {
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::sync::Arc;

    use axum::body::{Body, to_bytes};
    use http::Request;
    use tower::ServiceExt;
    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::domain::tag::{IntegerTagValue, Tag, TagKey, TagSet};
    use crate::infrastructure::metadata_index::InMemoryMetadataIndex;
    use crate::port::MetadataIndex;

    #[tokio::test]
    async fn tag_exists_rejects_key_disallowed_by_schema() {
        let schema = serde_yaml::from_str::<TagSchema>(
            "version: '0'\nallow_additional_tags: false\noptional:\n  allowed: { type: key_only }\n",
        )
        .unwrap();
        let index = Arc::new(InMemoryMetadataIndex::new());
        let controller = ContentController::new(
            schema,
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index),
        );

        for (key, expected_status) in [
            ("forbidden", StatusCode::BAD_REQUEST),
            ("allowed", StatusCode::OK),
        ] {
            let condition = format!(r#"[{{"kind":"tagExists","key":"{key}"}}]"#);
            let query = url::form_urlencoded::Serializer::new(String::new())
                .append_pair("condition", &condition)
                .finish();
            let response = controller
                .clone()
                .router()
                .oneshot(
                    Request::builder()
                        .uri(format!("/?{query}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), expected_status, "key: {key}");
        }
    }

    #[tokio::test]
    async fn media_type_and_tag_match_filter_contents() {
        let schema = serde_yaml::from_str::<TagSchema>(
            "version: '0'\nallow_additional_tags: false\noptional:\n  rating: { type: integer }\n",
        )
        .unwrap();
        let id = Uuid::new_v4();
        let content = domain::Content::new(
            id.try_into().unwrap(),
            domain::MediaType::Avif,
            Url::parse("https://example.com/content.avif").unwrap(),
            Url::parse("https://example.com/thumbnail.avif").unwrap(),
            TagSet::new(&[Tag::Integer {
                key: TagKey::new("rating").unwrap(),
                value: IntegerTagValue::new(5).unwrap(),
            }])
            .unwrap(),
            HashSet::new(),
        );
        let mut index = InMemoryMetadataIndex::new();
        index.add_contents(&[content]).unwrap();
        let index = Arc::new(index);
        let controller = ContentController::new(
            schema,
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index),
        );

        for (condition, expected_status, expected_count) in [
            (
                r#"[{"kind":"mediaTypeMatch","values":["image/avif"]}]"#,
                StatusCode::OK,
                Some(1),
            ),
            (
                r#"[{"kind":"mediaTypeMatch","values":[]}]"#,
                StatusCode::BAD_REQUEST,
                None,
            ),
            (
                r#"[{"kind":"tagMatch","key":"rating","values":["5"]}]"#,
                StatusCode::OK,
                Some(1),
            ),
            (
                r#"[{"kind":"tagMatch","key":"rating","values":["6"]}]"#,
                StatusCode::OK,
                Some(0),
            ),
        ] {
            let query = url::form_urlencoded::Serializer::new(String::new())
                .append_pair("condition", condition)
                .finish();
            let response = controller
                .clone()
                .router()
                .oneshot(
                    Request::builder()
                        .uri(format!("/?{query}"))
                        .body(Body::empty())
                        .unwrap(),
                )
                .await
                .unwrap();

            assert_eq!(response.status(), expected_status, "condition: {condition}");
            if let Some(expected_count) = expected_count {
                let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
                let page: serde_json::Value = serde_json::from_slice(&body).unwrap();
                assert_eq!(page["items"].as_array().unwrap().len(), expected_count);
                if expected_count == 1 {
                    assert_eq!(page["items"][0]["id"], id.to_string());
                }
            }
        }
    }
}

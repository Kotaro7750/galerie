use axum::Json;
use axum::body::Bytes;
use axum::extract::rejection::{JsonRejection, PathRejection, QueryRejection};
use axum::extract::{DefaultBodyLimit, Path, Query, State};
use axum::response::IntoResponse;
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart, TypedMultipartError};
use galerie_api::models::Tag as ApiTag;
use http::StatusCode;
use http::header::{CONTENT_TYPE, LOCATION};
use std::collections::HashSet;

use galerie_api::models::{
    BadRequestProblem, Content, ContentNotFoundProblem, ContentPage, DeleteContentPathParams,
    GetContentPathParams, InternalServerErrorProblem, SearchTerm, UnsupportedMediaTypeProblem,
    UpdateContentPathParams,
};

use crate::domain::search_condition::{FileFormatPredicate, TagPredicate, Term};
use crate::domain::tag::xmp::check_xmp_roundtrip;
use crate::domain::tag::{Tag, TagKey, TagSet};
use crate::domain::tag_schema::TagSchema;
use crate::usecase::{
    CreateContentUseCase, DeleteContentUseCase, GetContentUseCase, ListContentsUseCase,
    UpdateContentUseCase,
};
use crate::{domain, usecase};

#[derive(serde::Deserialize)]
pub(crate) struct ListContentsQueryParams {
    cursor: Option<String>,
    limit: Option<u8>,
    condition: Option<String>,
}

#[derive(TryFromMultipart)]
#[try_from_multipart(strict)]
struct CreateContentMultipart {
    content: FieldData<Bytes>,
    metadata: FieldData<Bytes>,
}

#[derive(Clone)]
pub(crate) struct ContentController {
    tag_schema: TagSchema,
    list_contents: ListContentsUseCase,
    get_content: GetContentUseCase,
    create_content: CreateContentUseCase,
    update_content: UpdateContentUseCase,
    delete_content: DeleteContentUseCase,
}

impl ContentController {
    pub(crate) fn new(
        tag_schema: TagSchema,
        list_contents: usecase::ListContentsUseCase,
        get_content: usecase::GetContentUseCase,
        create_content: usecase::CreateContentUseCase,
        update_content: usecase::UpdateContentUseCase,
        delete_content: usecase::DeleteContentUseCase,
    ) -> Self {
        ContentController {
            tag_schema,
            list_contents,
            get_content,
            create_content,
            update_content,
            delete_content,
        }
    }

    pub(crate) fn router(self) -> axum::Router {
        axum::Router::new()
            .route(
                "/{content_id}",
                axum::routing::get(Self::get_content)
                    .patch(Self::update_content)
                    .delete(Self::delete_content),
            )
            .route(
                "/",
                axum::routing::get(Self::list_contents).post(Self::create_content),
            )
            .layer(DefaultBodyLimit::max(100 * 1024 * 1024))
            .with_state(self)
    }

    async fn create_content(
        State(this): State<Self>,
        multipart: Result<TypedMultipart<CreateContentMultipart>, TypedMultipartError>,
    ) -> Result<impl IntoResponse, ContentControllerError> {
        let TypedMultipart(CreateContentMultipart { content, metadata }) =
            multipart.map_err(|e| match e {
                TypedMultipartError::InvalidRequest { .. } => {
                    ContentControllerError::UnsupportedMediaType
                }
                _ => ContentControllerError::BadRequest(e.to_string()),
            })?;

        let media_type = match content.metadata.content_type.as_deref() {
            Some("image/avif") => domain::MediaType::Avif,
            _ => return Err(ContentControllerError::UnsupportedMediaType),
        };
        if metadata.metadata.content_type.as_deref() != Some("application/json") {
            return Err(ContentControllerError::UnsupportedMediaType);
        }

        let tags = parse_input_tags(&metadata.contents, &this.tag_schema)?;
        let content = this
            .create_content
            .execute(media_type, &content.contents, tags)
            .await?;
        let location = format!("/api/v0/contents/{}", content.id().as_ref());
        Ok((
            StatusCode::CREATED,
            [(LOCATION, location)],
            Json(Content::from(content)),
        ))
    }

    pub(crate) async fn get_content(
        State(this): State<Self>,
        path: Result<Path<GetContentPathParams>, PathRejection>,
    ) -> Result<Json<Content>, ContentControllerError> {
        let id = path?.content_id.parse::<domain::ContentId>().map_err(|e| {
            ContentControllerError::BadRequest(format!("invalid contentId, err: {}", e))
        })?;

        let content = this.get_content.execute(id).await?;

        Ok(Json(content.into()))
    }

    async fn update_content(
        State(this): State<Self>,
        path: Result<Path<UpdateContentPathParams>, PathRejection>,
        input: Result<Json<ContentMetadataInput>, JsonRejection>,
    ) -> Result<Json<Content>, ContentControllerError> {
        let id = path?.content_id.parse::<domain::ContentId>().map_err(|e| {
            ContentControllerError::BadRequest(format!("invalid contentId, err: {}", e))
        })?;

        let Json(input) = input.map_err(|e| ContentControllerError::BadRequest(e.to_string()))?;
        let tags = normalize_input_tags(input, &this.tag_schema)?;

        let content = this.update_content.execute(id, tags).await?;

        Ok(Json(content.into()))
    }

    async fn delete_content(
        State(this): State<Self>,
        path: Result<Path<DeleteContentPathParams>, PathRejection>,
    ) -> Result<StatusCode, ContentControllerError> {
        let id = path?.content_id.parse::<domain::ContentId>().map_err(|e| {
            ContentControllerError::BadRequest(format!("invalid contentId, err: {}", e))
        })?;

        this.delete_content.execute(id).await?;

        Ok(StatusCode::NO_CONTENT)
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

        let (items, cursor) = this
            .list_contents
            .execute(
                limit.into(),
                query_params.cursor.clone(),
                Some(domain::search_condition::SearchCondition::new(
                    search_terms.as_slice(),
                )),
            )
            .await?;

        Ok(Json(ContentPage {
            items: items.into_iter().map(|c| c.into()).collect(),
            next_cursor: cursor,
        }))
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ContentMetadataInput {
    tags: Vec<ApiTag>,
}

fn parse_input_tags(metadata: &[u8], schema: &TagSchema) -> Result<TagSet, ContentControllerError> {
    let input: ContentMetadataInput = serde_json::from_slice(metadata)
        .map_err(|e| ContentControllerError::BadRequest(format!("Invalid metadata: {e}")))?;
    normalize_input_tags(input, schema)
}

fn normalize_input_tags(
    input: ContentMetadataInput,
    schema: &TagSchema,
) -> Result<TagSet, ContentControllerError> {
    let mut tags = Vec::with_capacity(input.tags.len());
    for input_tag in input.tags {
        let tag = match input_tag {
            ApiTag::KeyOnlyTag(v) => normalize_scalar(schema, &v.key, String::new(), |t| {
                matches!(t, Tag::KeyOnly { .. })
            })?,
            ApiTag::TextTag(v) => {
                normalize_scalar(schema, &v.key, v.value, |t| matches!(t, Tag::Text { .. }))?
            }
            ApiTag::IntegerTag(v) => normalize_scalar(schema, &v.key, v.value.to_string(), |t| {
                matches!(t, Tag::Integer { .. })
            })?,
            ApiTag::RealTag(v) => normalize_scalar(schema, &v.key, v.value.to_string(), |t| {
                matches!(t, Tag::Real { .. })
            })?,
            ApiTag::TextSetTag(v) => normalize_set(
                schema,
                &v.key,
                v.values.into_iter().map(|value| value.0).collect(),
                |t| matches!(t, Tag::TextSet { .. }),
            )?,
            ApiTag::IntegerSetTag(v) => normalize_set(
                schema,
                &v.key,
                v.values
                    .into_iter()
                    .map(|value| value.0.to_string())
                    .collect(),
                |t| matches!(t, Tag::IntegerSet { .. }),
            )?,
            ApiTag::RealSetTag(v) => normalize_set(
                schema,
                &v.key,
                v.values
                    .into_iter()
                    .map(|value| value.0.to_string())
                    .collect(),
                |t| matches!(t, Tag::RealSet { .. }),
            )?,
        };
        tags.push(tag);
    }
    if !schema.diagnose_tags_combination(&tags).is_empty() {
        return Err(ContentControllerError::BadRequest(
            "Required tags are missing".to_string(),
        ));
    }
    let tag_set = TagSet::new(&tags)
        .ok_or_else(|| ContentControllerError::BadRequest("Duplicate tag key".to_string()))?;
    if !check_xmp_roundtrip(&tag_set, schema) {
        return Err(ContentControllerError::BadRequest(
            "Tags cannot be represented as valid XMP".to_string(),
        ));
    }
    Ok(tag_set)
}

fn normalize_scalar(
    schema: &TagSchema,
    key: &str,
    value: String,
    matches_type: fn(&Tag) -> bool,
) -> Result<Tag, ContentControllerError> {
    let key = TagKey::new(key)
        .ok_or_else(|| ContentControllerError::BadRequest("Invalid tag key".to_string()))?;
    let tag = schema
        .normalize_tag(key, value)
        .map_err(|e| ContentControllerError::BadRequest(format!("Invalid tag value: {e:?}")))?;
    if !matches_type(&tag) {
        return Err(ContentControllerError::BadRequest(
            "Tag type does not match its definition".to_string(),
        ));
    }
    Ok(tag)
}

fn normalize_set(
    schema: &TagSchema,
    key: &str,
    values: Vec<String>,
    matches_type: fn(&Tag) -> bool,
) -> Result<Tag, ContentControllerError> {
    let key = TagKey::new(key)
        .ok_or_else(|| ContentControllerError::BadRequest("Invalid tag key".to_string()))?;
    let unique_values: HashSet<String> = values.iter().cloned().collect();
    if unique_values.len() != values.len() {
        return Err(ContentControllerError::BadRequest(
            "Duplicate tag set value".to_string(),
        ));
    }
    let tag = schema
        .normalize_set_tag(key, &unique_values)
        .map_err(|e| ContentControllerError::BadRequest(format!("Invalid tag value: {e:?}")))?;
    if !matches_type(&tag) {
        return Err(ContentControllerError::BadRequest(
            "Tag type does not match its definition".to_string(),
        ));
    }
    Ok(tag)
}

#[derive(Debug)]
pub(crate) enum ContentControllerError {
    BadRequest(String),
    UnsupportedMediaType,
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

            Self::UnsupportedMediaType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                [(CONTENT_TYPE, "application/problem+json")],
                Json(UnsupportedMediaTypeProblem {
                    detail: Some("Only multipart/form-data with image/avif content and application/json metadata is supported".to_string()),
                    ..UnsupportedMediaTypeProblem::new(
                        "Unsupported media type".to_string(),
                        StatusCode::UNSUPPORTED_MEDIA_TYPE.as_u16().into(),
                    )
                }),
            ).into_response(),

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
    use std::num::NonZeroU64;
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use axum::body::{Body, to_bytes};
    use http::Request;
    use tower::ServiceExt;
    use url::Url;
    use uuid::Uuid;

    use super::*;
    use crate::domain::tag::{IntegerTagValue, RealTagValue, Tag, TagKey, TagSet, TextTagValue};
    use crate::infrastructure::content_storage::filesystem::FileSystemContentStorage;
    use crate::infrastructure::metadata_index::InMemoryMetadataIndex;
    use crate::port::MetadataIndex;

    #[tokio::test]
    async fn patch_and_delete_update_storage_and_index() {
        let directory =
            std::env::temp_dir().join(format!("galerie-content-test-{}", Uuid::new_v4()));
        std::fs::create_dir(&directory).unwrap();
        let id: domain::ContentId = Uuid::new_v4().try_into().unwrap();
        let content_path = directory.join(format!("{}.avif", id.as_ref()));
        let xmp_path = directory.join(format!("{}.xmp", id.as_ref()));
        std::fs::write(&content_path, b"original").unwrap();
        std::fs::write(&xmp_path, concat!(
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\"><rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">",
            "<rdf:Description xmlns:galerie=\"galerie\" xmlns:other=\"urn:other\" galerie:old=\"old\" other:creator=\"Alice\"/>",
            "</rdf:RDF></x:xmpmeta>"
        )).unwrap();
        let storage = Arc::new(
            FileSystemContentStorage::new(
                directory.to_str().unwrap(),
                "https://example.com",
                "https://example.com",
            )
            .unwrap(),
        );
        let index = Arc::new(InMemoryMetadataIndex::new());
        index
            .add_contents(&[domain::Content::new(
                id,
                domain::MediaType::Avif,
                "https://example.com/a.avif".parse().unwrap(),
                "https://example.com/a.avif".parse().unwrap(),
                TagSet::new(&[]).unwrap(),
                HashSet::new(),
            )])
            .await
            .unwrap();
        let controller = ContentController::new(
            TagSchema::default(),
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index.clone()),
            CreateContentUseCase::new(storage.clone(), index.clone()),
            UpdateContentUseCase::new(storage.clone(), index.clone()),
            DeleteContentUseCase::new(storage, index.clone()),
        );
        let router = controller.router();
        let uri = format!("/{}", id.as_ref());
        let original_xmp = std::fs::read(&xmp_path).unwrap();
        let invalid = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(&uri)
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"tags":[{"key":"invalid-key","type":"text","value":"bad"}]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid.status(), StatusCode::BAD_REQUEST);
        assert_eq!(std::fs::read(&xmp_path).unwrap(), original_xmp);
        for (content_type, body) in [(Some("application/json"), "{"), (None, r#"{"tags":[]}"#)] {
            let mut request = Request::builder().method("PATCH").uri(&uri);
            if let Some(content_type) = content_type {
                request = request.header(CONTENT_TYPE, content_type);
            }
            let response = router
                .clone()
                .oneshot(request.body(Body::from(body)).unwrap())
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::BAD_REQUEST);
            assert_eq!(std::fs::read(&xmp_path).unwrap(), original_xmp);
        }
        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(&uri)
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"tags":[{"key":"title","type":"text","value":"new"}]}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            index.get_content(id).await.unwrap().tags().as_ref().len(),
            1
        );
        let xmp = std::fs::read_to_string(&xmp_path).unwrap();
        let metadata: xmp_toolkit::XmpMeta = xmp.parse().unwrap();
        assert!(metadata.property("galerie", "old").is_none());
        assert_eq!(metadata.property("galerie", "title").unwrap().value, "new");
        assert_eq!(
            metadata.property("urn:other", "creator").unwrap().value,
            "Alice"
        );
        assert_eq!(std::fs::read(&content_path).unwrap(), b"original");

        let response = router
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        assert!(!content_path.exists());
        assert!(!xmp_path.exists());
        assert!(matches!(
            index.get_content(id).await,
            Err(domain::Error::ContentNotFound)
        ));
        std::fs::remove_dir(directory).unwrap();
    }

    #[derive(Default)]
    struct RecordingStorage {
        content_input: Mutex<Option<(domain::MediaType, Vec<u8>)>>,
    }

    #[async_trait]
    impl crate::port::ContentStorage for RecordingStorage {
        async fn create_content_file(
            &self,
            _: domain::ContentId,
            media_type: domain::MediaType,
            content_bytes: &[u8],
        ) -> Result<(Url, Url), domain::Error> {
            *self.content_input.lock().unwrap() = Some((media_type, content_bytes.to_vec()));
            Ok((
                Url::parse("https://example.com/content.avif").unwrap(),
                Url::parse("https://example.com/thumbnail.avif").unwrap(),
            ))
        }

        async fn create_xmp_sidecar(
            &self,
            _: domain::ContentId,
            _: &str,
        ) -> Result<(), domain::Error> {
            Ok(())
        }

        async fn delete_content_file(
            &self,
            _: domain::ContentId,
            _: domain::MediaType,
        ) -> Result<(), domain::Error> {
            unreachable!()
        }
        async fn delete_xmp_sidecar(&self, _: domain::ContentId) -> Result<(), domain::Error> {
            unreachable!()
        }
        async fn get_xmp_sidecar(&self, _: domain::ContentId) -> Result<String, domain::Error> {
            unreachable!()
        }
        async fn replace_xmp_sidecar(
            &self,
            _: domain::ContentId,
            _: &str,
        ) -> Result<(), domain::Error> {
            unreachable!()
        }

        async fn scan_contents(
            &self,
            _: &TagSchema,
            _: NonZeroU64,
            _: Option<domain::ContentId>,
        ) -> Result<(Vec<domain::Content>, Option<domain::ContentId>), domain::Error> {
            unreachable!()
        }
    }

    struct UnusedStorage;

    #[async_trait]
    impl crate::port::ContentStorage for UnusedStorage {
        async fn create_content_file(
            &self,
            _: domain::ContentId,
            _: domain::MediaType,
            _: &[u8],
        ) -> Result<(url::Url, url::Url), domain::Error> {
            unreachable!()
        }

        async fn create_xmp_sidecar(
            &self,
            _: domain::ContentId,
            _: &str,
        ) -> Result<(), domain::Error> {
            unreachable!()
        }

        async fn delete_content_file(
            &self,
            _: domain::ContentId,
            _: domain::MediaType,
        ) -> Result<(), domain::Error> {
            unreachable!()
        }
        async fn delete_xmp_sidecar(&self, _: domain::ContentId) -> Result<(), domain::Error> {
            unreachable!()
        }
        async fn get_xmp_sidecar(&self, _: domain::ContentId) -> Result<String, domain::Error> {
            unreachable!()
        }
        async fn replace_xmp_sidecar(
            &self,
            _: domain::ContentId,
            _: &str,
        ) -> Result<(), domain::Error> {
            unreachable!()
        }

        async fn scan_contents(
            &self,
            _: &TagSchema,
            _: NonZeroU64,
            _: Option<domain::ContentId>,
        ) -> Result<(Vec<domain::Content>, Option<domain::ContentId>), domain::Error> {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn create_content_maps_request_to_use_case_and_returns_created_response() {
        let storage = Arc::new(RecordingStorage::default());
        let index = Arc::new(InMemoryMetadataIndex::new());
        let schema: TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\nrequired:\n  rating: { type: integer }\n",
        )
        .unwrap();
        let controller = ContentController::new(
            schema,
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index.clone()),
            CreateContentUseCase::new(storage.clone(), index.clone()),
            UpdateContentUseCase::new(storage.clone(), index.clone()),
            DeleteContentUseCase::new(storage.clone(), index),
        );
        let body = concat!(
            "--boundary\r\nContent-Disposition: form-data; name=\"content\"; filename=\"test.avif\"\r\nContent-Type: image/avif\r\n\r\n",
            "opaque-content-bytes",
            "\r\n--boundary\r\nContent-Disposition: form-data; name=\"metadata\"\r\nContent-Type: application/json\r\n\r\n",
            "{\"tags\":[{\"key\":\"rating\",\"type\":\"integer\",\"value\":5}]}",
            "\r\n--boundary--\r\n"
        );
        let response = axum::Router::new()
            .nest("/api/v0/contents", controller.router())
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v0/contents")
                    .header(CONTENT_TYPE, "multipart/form-data; boundary=boundary")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
        let location = response
            .headers()
            .get(LOCATION)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let content: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(content["mediaType"], "image/avif");
        assert_eq!(content["diagnostics"], serde_json::json!([]));
        assert_eq!(content["tags"][0]["value"], 5);
        let id = content["id"].as_str().unwrap();
        assert_eq!(location, format!("/api/v0/contents/{id}"));
        assert_eq!(
            *storage.content_input.lock().unwrap(),
            Some((domain::MediaType::Avif, b"opaque-content-bytes".to_vec()))
        );
    }

    #[tokio::test]
    async fn create_content_rejects_invalid_multipart_parts_and_media_type() {
        let index = Arc::new(InMemoryMetadataIndex::new());
        let controller = ContentController::new(
            TagSchema::default(),
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index.clone()),
            CreateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            UpdateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            DeleteContentUseCase::new(Arc::new(UnusedStorage), index),
        );
        let router = controller.router();
        let part = |name: &str, content_type: &str, body: &str| {
            format!(
                "--boundary\r\nContent-Disposition: form-data; name=\"{name}\"\r\nContent-Type: {content_type}\r\n\r\n{body}\r\n"
            )
        };
        let content = part("content", "image/avif", "bytes");
        let metadata = part("metadata", "application/json", r#"{"tags":[]}"#);
        for (body, expected) in [
            (
                format!("{content}{content}{metadata}--boundary--\r\n"),
                StatusCode::BAD_REQUEST,
            ),
            (
                format!(
                    "{content}{metadata}{}--boundary--\r\n",
                    part("extra", "text/plain", "x")
                ),
                StatusCode::BAD_REQUEST,
            ),
            (
                format!("{content}--boundary--\r\n"),
                StatusCode::BAD_REQUEST,
            ),
            (
                format!(
                    "{}{}--boundary--\r\n",
                    part("content", "image/png", "bytes"),
                    metadata
                ),
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
            ),
        ] {
            let response = router
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri("/")
                        .header(CONTENT_TYPE, "multipart/form-data; boundary=boundary")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), expected);
        }
    }

    #[test]
    fn parse_input_tags_reports_controller_validation_errors() {
        let schema: TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\nrequired:\n  rating: { type: integer }\noptional:\n  weights: { type: real_set }\n"
        ).unwrap();
        for (metadata, expected_message) in [
            (r#"{"tags":[]}"#, "Required tags are missing"),
            (
                r#"{"tags":[{"key":"rating","type":"text","value":"5"}]}"#,
                "Tag type does not match its definition",
            ),
            (
                r#"{"tags":[{"key":"rating","type":"integer","value":5},{"key":"rating","type":"integer","value":6}]}"#,
                "Duplicate tag key",
            ),
            (
                r#"{"tags":[{"key":"rating","type":"integer","value":5},{"key":"weights","type":"realSet","values":[1.0,1.0]}]}"#,
                "Duplicate tag set value",
            ),
        ] {
            let result = parse_input_tags(metadata.as_bytes(), &schema);
            assert!(
                matches!(result, Err(ContentControllerError::BadRequest(message)) if message == expected_message),
                "{metadata}"
            );
        }
    }

    #[test]
    fn parse_input_tags_accepts_empty_set_when_schema_allows_it() {
        let schema: TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\noptional:\n  labels: { type: text_set }\n",
        )
        .unwrap();
        let result = parse_input_tags(
            br#"{"tags":[{"key":"labels","type":"textSet","values":[]}]}"#,
            &schema,
        );
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn parse_input_tags_accepts_all_generated_tag_variants() {
        let schema: TagSchema = serde_yaml::from_str(
            "version: '0'\nallow_additional_tags: false\noptional:\n  flag: { type: key_only }\n  title: { type: text }\n  rating: { type: integer }\n  score: { type: real }\n  labels: { type: text_set }\n  ranks: { type: integer_set }\n  weights: { type: real_set }\n",
        ).unwrap();
        let metadata = br#"{"tags":[{"key":"flag","type":"keyOnly"},{"key":"title","type":"text","value":"hello"},{"key":"rating","type":"integer","value":5},{"key":"score","type":"real","value":1.5},{"key":"labels","type":"textSet","values":["a"]},{"key":"ranks","type":"integerSet","values":[1,2]},{"key":"weights","type":"realSet","values":[1.5,2.5]}]}"#;
        let tags = parse_input_tags(metadata, &schema).unwrap();
        let key = |name| TagKey::new(name).unwrap();
        let expected = TagSet::new(&[
            Tag::KeyOnly { key: key("flag") },
            Tag::Text {
                key: key("title"),
                value: TextTagValue::new("hello").unwrap(),
            },
            Tag::Integer {
                key: key("rating"),
                value: IntegerTagValue::new(5).unwrap(),
            },
            Tag::Real {
                key: key("score"),
                value: RealTagValue::new(1.5).unwrap(),
            },
            Tag::TextSet {
                key: key("labels"),
                values: HashSet::from([TextTagValue::new("a").unwrap()]),
            },
            Tag::IntegerSet {
                key: key("ranks"),
                values: HashSet::from([
                    IntegerTagValue::new(1).unwrap(),
                    IntegerTagValue::new(2).unwrap(),
                ]),
            },
            Tag::RealSet {
                key: key("weights"),
                values: HashSet::from([
                    RealTagValue::new(1.5).unwrap(),
                    RealTagValue::new(2.5).unwrap(),
                ]),
            },
        ])
        .unwrap();
        assert_eq!(tags, expected);
    }

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
            GetContentUseCase::new(index.clone()),
            CreateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            UpdateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            DeleteContentUseCase::new(Arc::new(UnusedStorage), index),
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
        let index = InMemoryMetadataIndex::new();
        index.add_contents(&[content]).await.unwrap();
        let index = Arc::new(index);
        let controller = ContentController::new(
            schema,
            ListContentsUseCase::new(index.clone()),
            GetContentUseCase::new(index.clone()),
            CreateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            UpdateContentUseCase::new(Arc::new(UnusedStorage), index.clone()),
            DeleteContentUseCase::new(Arc::new(UnusedStorage), index),
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

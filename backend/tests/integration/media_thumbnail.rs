use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use galarie_backend::media::MediaType;

use super::support::IntegrationTestApp;

#[tokio::test]
async fn thumbnail_returns_binary_payload_with_headers() {
    let app = IntegrationTestApp::new().await;
    let media = app.media_by_type(MediaType::Image).await;

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("/api/v1/media/{}/thumbnail?size=small", media.id))
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::OK);
    assert!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.starts_with("image/"))
            .unwrap_or(false),
        "thumbnail response should declare image content-type"
    );

    let bytes = response.into_body().collect().await.expect("body").to_bytes();
    assert!(!bytes.is_empty(), "thumbnail body should not be empty");
}

#[tokio::test]
async fn thumbnail_missing_resource_uses_error_envelope() {
    let app = IntegrationTestApp::new().await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media/missing-thumb/thumbnail")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let json: Value = serde_json::from_slice(&body).expect("json payload");
    assert_eq!(json["error"]["code"], "RESOURCE_NOT_FOUND");
}

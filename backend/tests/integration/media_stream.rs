use std::path::Path;

use axum::{
    body::Body,
    http::{
        Method, Request, StatusCode,
        header::{ACCEPT_RANGES, CONTENT_RANGE, CONTENT_TYPE, ETAG},
    },
};
use galarie_backend::media::MediaType;
use http_body_util::BodyExt;
use tokio::fs;

use super::support::IntegrationTestApp;

#[tokio::test]
async fn stream_returns_original_bytes_with_headers() {
    let app = IntegrationTestApp::new().await;
    let media = app.media_by_type(MediaType::Image).await;

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("/api/v1/media/{}/stream", media.id))
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get(ACCEPT_RANGES).unwrap(),
        "bytes",
        "range header required for seeking"
    );
    let content_type = response.headers().get(CONTENT_TYPE).unwrap();
    assert!(
        content_type.to_str().unwrap_or_default().starts_with("image/"),
        "expected image content-type, got {content_type:?}"
    );
    let etag = response.headers().get(ETAG).expect("etag header present");
    assert!(
        !etag.is_empty(),
        "stream responses must emit cache validators"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body bytes")
        .to_bytes();

    let expected_path = app.media_root.join(Path::new(&media.relative_path));
    let expected = fs::read(expected_path)
        .await
        .expect("read sample media file");
    assert_eq!(body, expected);
}

#[tokio::test]
async fn missing_media_returns_not_found() {
    let app = IntegrationTestApp::new().await;
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media/deadbeef/stream")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response.into_body().collect().await.expect("body");
    let json: serde_json::Value = serde_json::from_slice(&body.to_bytes()).expect("json payload");
    assert_eq!(json["error"]["code"], "RESOURCE_NOT_FOUND");
    assert!(
        json["error"]["message"]
            .as_str()
            .unwrap_or_default()
            .contains("media"),
        "not-found errors should mention the missing media"
    );
}

#[tokio::test]
async fn stream_supports_partial_content_requests() {
    let app = IntegrationTestApp::new().await;
    let media = app.media_by_type(MediaType::Image).await;

    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("/api/v1/media/{}/stream?disposition=inline", media.id))
        .header("Range", "bytes=0-9")
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::PARTIAL_CONTENT);
    assert_eq!(response.headers().get(ACCEPT_RANGES).unwrap(), "bytes");
    let content_range = response.headers().get(CONTENT_RANGE).unwrap();
    assert!(
        content_range
            .to_str()
            .unwrap_or_default()
            .starts_with("bytes 0-9/"),
        "content-range should describe partial bytes"
    );

    let bytes = response.into_body().collect().await.expect("body").to_bytes();
    assert_eq!(bytes.len(), 10);
}

#[tokio::test]
async fn stream_invalid_disposition_returns_validation_error() {
    let app = IntegrationTestApp::new().await;
    let media = app.media_by_type(MediaType::Image).await;
    let request = Request::builder()
        .method(Method::GET)
        .uri(format!("/api/v1/media/{}/stream?disposition=download", media.id))
        .body(Body::empty())
        .expect("request");

    let response = app.request(request).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("json payload");
    assert_eq!(json["error"]["code"], "VALIDATION_FAILED");
}

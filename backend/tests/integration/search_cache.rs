use std::time::{Duration, Instant};

use super::support::{build_router_with_cache, sample_media_root};
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use tempfile::tempdir;
use tower::ServiceExt;

#[tokio::test]
async fn cache_miss_rebuilds_and_search_responds_under_one_second() {
    let media_root = sample_media_root();
    let cache_dir = tempdir().expect("temp cache dir");
    let rebuild_start = Instant::now();
    let (app, _) = build_router_with_cache(media_root, cache_dir.path().to_path_buf());
    let rebuild_elapsed = rebuild_start.elapsed();
    assert!(
        rebuild_elapsed <= Duration::from_secs(1),
        "expected cache rebuild within 1s for sample dataset, took {rebuild_elapsed:?}"
    );

    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/media?tags=sunset")
        .body(Body::empty())
        .expect("request");

    let response = app.clone().oneshot(request).await.expect("router response");
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "search endpoint should respond successfully"
    );

    let body = response.into_body().collect().await.expect("body");
    let json: Value = serde_json::from_slice(&body.to_bytes()).expect("json payload");
    assert!(
        json["items"].is_array(),
        "search response should contain items array"
    );
}

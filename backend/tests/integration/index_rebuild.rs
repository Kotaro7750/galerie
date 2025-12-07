use std::time::Duration;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use tempfile::tempdir;
use tokio::time::timeout;
use tower::ServiceExt;

use super::support::{build_router_with_cache, sample_media_root};

#[tokio::test]
async fn rebuild_endpoint_triggers_rescan() {
    let media_root = sample_media_root();
    let cache_dir = tempdir().expect("temp cache dir");
    let (router, cache) = build_router_with_cache(media_root, cache_dir.path().to_path_buf());

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/index/rebuild")
        .body(Body::empty())
        .expect("request");

    let response = router
        .clone()
        .oneshot(request)
        .await
        .expect("router response");

    assert_eq!(response.status(), StatusCode::ACCEPTED);

    // wait for background rebuild to populate cache
    timeout(Duration::from_secs(2), async {
        loop {
            if cache.read_snapshot().await.media.len() >= 3 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("rebuild should complete");
}

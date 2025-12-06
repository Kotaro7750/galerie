use std::{path::PathBuf, sync::Arc, time::Duration};

use axum::{body::Body, http::{Method, Request, StatusCode}};
use galarie_backend::{
    cache::{CacheSnapshot, CacheStore},
    config::{AppConfig, LogConfig, OtelConfig},
    routes::{self, AppState},
};
use tempfile::tempdir;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tower::ServiceExt;

use super::support::sample_media_root;

#[tokio::test]
async fn rebuild_endpoint_triggers_rescan() {
    let media_root = sample_media_root();
    let cache_dir = tempdir().expect("temp cache dir");
    let config = Arc::new(test_config(
        media_root.clone(),
        cache_dir.path().to_path_buf(),
    ));

    let cache_store = Arc::new(CacheStore::new(cache_dir.path()));
    let snapshot_state = Arc::new(RwLock::new(CacheSnapshot::new(Vec::new())));
    let state = AppState::new(config, cache_store.clone(), snapshot_state.clone());
    let router = routes::router(state);

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
            if snapshot_state.read().await.media.len() >= 3 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    })
    .await
    .expect("rebuild should complete");

    // ensure cache persisted to disk
    let snapshot = cache_store
        .load()
        .expect("load cache")
        .expect("snapshot written");
    assert!(snapshot.media.len() >= 3);
}

fn test_config(media_root: PathBuf, cache_dir: PathBuf) -> AppConfig {
    AppConfig {
        media_root,
        cache_dir,
        listen_addr: "127.0.0.1:0".parse().unwrap(),
        environment: "test".into(),
        otel: OtelConfig {
            endpoint: None,
            service_name: "test-backend".into(),
            disable_traces: true,
            disable_logs: true,
        },
        log: LogConfig { level: "info".into() },
        cors_allowed_origins: Vec::new(),
        frontend_dist_dir: None,
    }
}

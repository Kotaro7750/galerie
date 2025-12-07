use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{Router, body::Body, http::Request};
use galarie_backend::{
    cache::Cache,
    config::{AppConfig, LogConfig, OtelConfig},
    media::{MediaFile, MediaType},
    routes::{self, AppState},
};
use tempfile::tempdir;
use tower::ServiceExt;

pub struct IntegrationTestApp {
    pub router: Router,
    pub cache: Arc<Cache>,
    pub media_root: PathBuf,
}

impl IntegrationTestApp {
    pub async fn new() -> Self {
        let media_root = sample_media_root();
        let cache_dir = tempdir().expect("temp cache dir");
        let config = Arc::new(test_config(
            media_root.clone(),
            cache_dir.path().to_path_buf(),
        ));

        let cache = Arc::new(
            Cache::new(config.media_root.clone(), config.cache_dir.clone()).expect("cache init"),
        );
        let state = AppState::new(config.clone(), cache.clone());
        let router = routes::router(state);

        Self {
            router,
            cache,
            media_root,
        }
    }

    pub async fn media_by_type(&self, media_type: MediaType) -> MediaFile {
        let snapshot = self.cache.read_snapshot().await;
        snapshot
            .media
            .iter()
            .find(|item| item.media_type == media_type)
            .cloned()
            .expect("media of requested type available")
    }

    pub async fn request(&self, request: Request<Body>) -> axum::response::Response {
        self.router
            .clone()
            .oneshot(request)
            .await
            .expect("router response")
    }
}

pub fn sample_media_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sample-media")
}

pub fn test_config(media_root: PathBuf, cache_dir: PathBuf) -> AppConfig {
    AppConfig {
        media_root,
        cache_dir,
        listen_addr: SocketAddr::from(([127, 0, 0, 1], 0)),
        environment: "test".into(),
        otel: OtelConfig {
            endpoint: None,
            service_name: "test-backend".into(),
            disable_traces: true,
            disable_logs: true,
        },
        log: LogConfig {
            level: "info".into(),
        },
        cors_allowed_origins: Vec::new(),
        frontend_dist_dir: None,
    }
}

pub fn build_router_with_cache(
    media_root: PathBuf,
    cache_dir: PathBuf,
) -> (Router, Arc<Cache>) {
    let config = Arc::new(test_config(media_root.clone(), cache_dir.clone()));
    let cache = Arc::new(
        Cache::new(media_root, cache_dir).expect("cache init"),
    );
    let state = AppState::new(config, cache.clone());
    let router = routes::router(state);
    (router, cache)
}

use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};

use axum::{Router, body::Body, http::Request};
use galarie_backend::{
    cache::{CacheSnapshot, CacheStore},
    config::{AppConfig, LogConfig, OtelConfig},
    indexer::{Indexer, MediaFile, MediaType},
    routes::{self, AppState},
};
use tempfile::tempdir;
use tokio::sync::RwLock;
use tower::ServiceExt;

pub struct IntegrationTestApp {
    pub router: Router,
    pub snapshot: Arc<RwLock<CacheSnapshot>>,
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

        let cache_store = Arc::new(CacheStore::new(cache_dir.path()));
        let snapshot = cache_store
            .load_or_rebuild(|| Indexer::scan_once(&media_root))
            .expect("cache rebuild");
        let snapshot_state = Arc::new(RwLock::new(snapshot));

        let state = AppState::new(config, cache_store, snapshot_state.clone());
        let router = routes::router(state);

        Self {
            router,
            snapshot: snapshot_state,
            media_root,
        }
    }

    pub async fn media_by_type(&self, media_type: MediaType) -> MediaFile {
        let snapshot = self.snapshot.read().await;
        snapshot
            .media
            .iter()
            .find(|item| item.media_type == media_type)
            .cloned()
            .expect("media of requested type available")
    }

    pub async fn request(&self, request: Request<Body>) -> axum::response::Response {
        self.router.clone().oneshot(request).await.expect("router response")
    }
}

pub fn sample_media_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sample-media")
}

fn test_config(media_root: PathBuf, cache_dir: PathBuf) -> AppConfig {
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
        log: LogConfig { level: "info".into() },
        cors_allowed_origins: Vec::new(),
        frontend_dist_dir: None,
    }
}

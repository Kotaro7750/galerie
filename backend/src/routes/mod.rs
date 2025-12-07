mod cors;
mod state;
mod telemetry;

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    middleware,
    routing::{get, post},
};
use serde::Serialize;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::instrument;

use crate::api::{self, ApiResponse, ApiResult, search, stream, thumbnails};

pub use state::AppState;
use telemetry::{HttpMakeSpan, LogOnRequest, LogOnResponse};

/// Build the Axum router with shared layers and routes.
pub fn router(state: AppState) -> Router {
    let cors_layer = cors::build_cors_layer(&state.config.cors_allowed_origins);

    let api_routes = Router::new()
        .route("/media", get(search::media_search))
        .route("/media/{id}/thumbnail", get(thumbnails::media_thumbnail))
        .route("/media/{id}/stream", get(stream::media_stream))
        .route("/index/rebuild", post(trigger_rebuild))
        .layer(cors_layer)
        .fallback(api::fallback_handler)
        .layer(middleware::from_fn(api::ensure_error_envelope))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(HttpMakeSpan)
                .on_request(LogOnRequest)
                .on_response(LogOnResponse),
        );

    let router = Router::new()
        .route("/healthz", get(healthz))
        .nest("/api/v1", api_routes)
        .with_state(state.clone());

    if let Some(frontend_dist_dir) = &state.config.frontend_dist_dir {
        let frontend_service = ServeDir::new(frontend_dist_dir)
            .fallback(ServeFile::new(frontend_dist_dir.join("index.html")));

        router.nest_service("/ui", frontend_service)
    } else {
        router
    }
}

/// JSON payload returned by `/healthz`.
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    media_root: String,
    cache_dir: String,
    uptime_seconds: f64,
    cache_items: usize,
    cache_generated_at: String,
}

#[instrument(skip(state))]
async fn healthz(State(state): State<AppState>) -> ApiResult<HealthResponse> {
    let snapshot = state.cache.read_snapshot().await;
    Ok(Json(HealthResponse {
        status: "ok",
        media_root: state.config.media_root.display().to_string(),
        cache_dir: state.config.cache_dir.display().to_string(),
        uptime_seconds: state.boot_instant.elapsed().as_secs_f64(),
        cache_items: snapshot.media.len(),
        cache_generated_at: snapshot.generated_at.to_rfc3339(),
    }))
}

#[instrument(skip(state))]
async fn trigger_rebuild(State(state): State<AppState>) -> ApiResponse<serde_json::Value> {
    state.cache.trigger_rebuild().await?;

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({"status": "queued"})),
    ))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::{
//         body::Body,
//         http::{Method, Request},
//     };
//     use http_body_util::BodyExt;
//     use serde_json::Value;
//     use std::path::PathBuf;
//     use std::sync::Arc;
//     use std::{fs, os::unix::fs::PermissionsExt, time::Duration};
//     use tempfile::tempdir;
//     use tokio::sync::RwLock;
//     use tokio::time::timeout;
//     use tower::ServiceExt;
//
//     use crate::cache::{Cache, CacheSnapshot, CacheStore};
//     use crate::config::{AppConfig, LogConfig, OtelConfig};
//
//     fn sample_media_root() -> PathBuf {
//         PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sample-media")
//     }
//
//     fn test_config(media_root: PathBuf, cache_dir: PathBuf) -> AppConfig {
//         AppConfig {
//             media_root,
//             cache_dir,
//             listen_addr: "127.0.0.1:0".parse().unwrap(),
//             environment: "test".into(),
//             otel: OtelConfig {
//                 endpoint: None,
//                 service_name: "test-service".into(),
//                 disable_traces: true,
//                 disable_logs: true,
//             },
//             log: LogConfig {
//                 level: "info".into(),
//             },
//             cors_allowed_origins: Vec::new(),
//             frontend_dist_dir: None,
//         }
//     }
//
//     fn test_state() -> AppState {
//         let media_root = sample_media_root();
//         let cache_dir = tempdir().unwrap();
//         let config = Arc::new(test_config(
//             media_root.clone(),
//             cache_dir.path().to_path_buf(),
//         ));
//
//         let initial_snapshot = CacheSnapshot::new(Vec::new());
//
//         AppState::new(
//             config,
//             Arc::new(Cache::new_with_snapshot(
//                 media_root,
//                 cache_dir.path(),
//                 initial_snapshot,
//             )),
//         )
//     }
//
//     async fn post_rebuild(app: &mut Router) -> StatusCode {
//         let request = Request::builder()
//             .method(Method::POST)
//             .uri("/api/v1/index/rebuild")
//             .body(Body::empty())
//             .unwrap();
//
//         app.clone().oneshot(request).await.unwrap().status()
//     }
//
//     #[tokio::test]
//     async fn rebuild_endpoint_updates_cache_snapshot() {
//         let state = test_state();
//         let mut app = router(state);
//
//         let status = post_rebuild(&mut app).await;
//         assert_eq!(status, StatusCode::ACCEPTED);
//
//         timeout(Duration::from_secs(2), async {
//             loop {
//                 if snapshot_state.read().await.media.len() >= 3 {
//                     break;
//                 }
//                 tokio::time::sleep(Duration::from_millis(20)).await;
//             }
//         })
//         .await
//         .expect("rebuild did not complete in time");
//     }
//
//     #[cfg(unix)]
//     #[tokio::test]
//     async fn rebuild_endpoint_handles_persist_failure() {
//         let media_root = sample_media_root();
//         let cache_dir = tempdir().unwrap();
//         let config = Arc::new(test_config(media_root, cache_dir.path().to_path_buf()));
//         let cache_store = Arc::new(CacheStore::new(cache_dir.path()));
//         let initial_snapshot = CacheSnapshot::new(Vec::new());
//         let snapshot_state = Arc::new(RwLock::new(initial_snapshot));
//
//         fs::set_permissions(cache_dir.path(), fs::Permissions::from_mode(0o555)).unwrap();
//
//         let state = AppState::new(config, cache_store, snapshot_state.clone());
//         let mut app = router(state);
//
//         let status = post_rebuild(&mut app).await;
//         assert_eq!(status, StatusCode::ACCEPTED);
//
//         tokio::time::sleep(Duration::from_millis(200)).await;
//         assert_eq!(snapshot_state.read().await.media.len(), 0);
//
//         fs::set_permissions(cache_dir.path(), fs::Permissions::from_mode(0o755)).unwrap();
//     }
//
//     #[tokio::test]
//     async fn fallback_returns_standard_error() {
//         let media_root = sample_media_root();
//         let cache_dir = tempdir().unwrap();
//         let config = Arc::new(test_config(media_root, cache_dir.path().to_path_buf()));
//         let cache_store = Arc::new(CacheStore::new(cache_dir.path()));
//         let snapshot_state = Arc::new(RwLock::new(CacheSnapshot::new(Vec::new())));
//
//         let state = AppState::new(config, cache_store, snapshot_state);
//         let app = router(state);
//
//         let response = app
//             .clone()
//             .oneshot(
//                 Request::builder()
//                     .method(Method::GET)
//                     .uri("/api/v1/missing")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//
//         assert_eq!(response.status(), StatusCode::NOT_FOUND);
//         let body = response.into_body().collect().await.unwrap().to_bytes();
//         let json: Value = serde_json::from_slice(&body).unwrap();
//         assert_eq!(json["error"]["code"], "RESOURCE_NOT_FOUND");
//     }
//
//     #[tokio::test]
//     async fn method_not_allowed_returns_standard_error() {
//         let media_root = sample_media_root();
//         let cache_dir = tempdir().unwrap();
//         let config = Arc::new(test_config(media_root, cache_dir.path().to_path_buf()));
//         let cache_store = Arc::new(CacheStore::new(cache_dir.path()));
//         let snapshot_state = Arc::new(RwLock::new(CacheSnapshot::new(Vec::new())));
//
//         let state = AppState::new(config, cache_store, snapshot_state);
//         let app = router(state);
//
//         let response = app
//             .clone()
//             .oneshot(
//                 Request::builder()
//                     .method(Method::GET)
//                     .uri("/api/v1/index/rebuild")
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap();
//
//         assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
//         let body = response.into_body().collect().await.unwrap().to_bytes();
//         let json: Value = serde_json::from_slice(&body).unwrap();
//         assert_eq!(json["error"]["code"], "METHOD_NOT_ALLOWED");
//     }
// }

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use galarie_backend::{
    cache::Cache,
    config::AppConfig,
    o11y,
    routes::{self, AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Arc::new(AppConfig::load()?);
    let _telemetry = o11y::TelemetryGuard::init(&config)?;

    tracing::info!("starting Galarie backend with config {:?}", config);

    let media_root_for_cache = config.media_root.clone();

    let cache = Arc::new(Cache::new(media_root_for_cache, config.cache_dir.clone())?);

    let state = AppState::new(config.clone(), cache.clone());
    let abort_sync_loop = cache.launch_sync_loop(Duration::from_secs(30));

    let listener = tokio::net::TcpListener::bind(config.listen_addr).await?;
    tracing::info!(addr = %config.listen_addr, "HTTP server listening");

    axum::serve(listener, routes::router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // Ensure the indexer task stops when the server exits.
    abort_sync_loop();

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("failed to install signal handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}

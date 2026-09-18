use std::net::SocketAddr;
use std::num::NonZeroU64;
use std::sync::Arc;
use tokio::signal;

use ::config::Config;
use axum::{Router, middleware};
use controller::content::ContentController;
use controller::content_access::ContentAccessController;
use usecase::{ClearContentAccessUseCase, ConfigureContentAccessUseCase, GetContentUseCase};

use crate::config::GalerieConfig;
use crate::infrastructure::metadata_index::InMemoryMetadataIndex;
use crate::port::MetadataIndex;
use crate::usecase::ListContentsUseCase;

mod config;
mod controller;
mod domain;
mod infrastructure;
mod observability;
mod port;
mod usecase;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    observability::init_logging();

    let config = Config::builder()
        .add_source(
            ::config::Environment::with_prefix("GALERIE")
                .separator("__")
                .prefix_separator("_"),
        )
        .build()?
        .try_deserialize::<GalerieConfig>()?;

    config.validate().await?;

    let content_storage = config.content_storage().construct_content_storage().await?;
    let mut metadata_index = InMemoryMetadataIndex::new();

    let mut cursor = None;
    loop {
        let (contents, next_cursor) = content_storage
            .scan_contents(NonZeroU64::new(100).unwrap(), cursor)
            .await?;
        metadata_index.add_contents(&contents)?;
        cursor = next_cursor;

        if cursor.is_none() {
            break;
        }
    }
    let metadata_index = Arc::new(metadata_index);
    let content_access_configurator = config
        .content_access()
        .construct_content_access_configurator()?;

    let contents_controller = ContentController::new(
        ListContentsUseCase::new(metadata_index.clone()),
        GetContentUseCase::new(metadata_index.clone()),
    );
    let content_access_controller = ContentAccessController::new(
        ConfigureContentAccessUseCase::new(content_access_configurator.clone()),
        ClearContentAccessUseCase::new(content_access_configurator),
    );

    let api_v0_router = Router::new()
        .nest("/contents", contents_controller.router())
        .nest("/content-access", content_access_controller.router());
    let api_v0_router = match config.authorization().construct_auth_layer().await? {
        Some(auth_layer) => api_v0_router.layer(auth_layer),
        None => api_v0_router,
    };
    let app = Router::new()
        .nest("/api/v0", api_v0_router)
        .layer(middleware::from_fn(observability::access_log))
        .route("/health", axum::routing::get(|| async { "OK" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.listen_address()).await?;
    tracing::info!("Server is running on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    tracing::info!("Server has been shut down gracefully.");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Signal received, starting graceful shutdown...");
}

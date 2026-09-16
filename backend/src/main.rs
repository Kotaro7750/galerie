use std::num::NonZeroU64;
use std::sync::Arc;

use ::config::Config;
use axum::Router;
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
mod port;
mod usecase;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let app = Router::new().nest("/api/v0", api_v0_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.listen_address()).await?;
    Ok(axum::serve(listener, app).await?)
}

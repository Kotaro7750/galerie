use std::num::NonZeroU64;
use std::sync::Arc;

use axum::Router;
use config::Config;
use controller::ContentController;
use usecase::GetContentUseCase;

use crate::galerie_config::GalerieConfig;
use crate::infrastructure::metadata_index::InMemoryMetadataIndex;
use crate::port::MetadataIndex;
use crate::usecase::ListContentsUseCase;

mod controller;
mod domain;
#[path = "config.rs"]
mod galerie_config;
mod infrastructure;
mod port;
mod usecase;

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(
            config::Environment::with_prefix("GALERIE")
                .separator("__")
                .prefix_separator("_"),
        )
        .build()
        .unwrap()
        .try_deserialize::<GalerieConfig>()
        .unwrap();

    let content_storage = config
        .content_storage()
        .construct_content_storage()
        .await
        .unwrap();
    let mut metadata_index = InMemoryMetadataIndex::new();

    let mut cursor = None;
    loop {
        let (contents, next_cursor) = content_storage
            .scan_contents(NonZeroU64::new(100).unwrap(), cursor)
            .await
            .unwrap();
        metadata_index.add_contents(&contents).unwrap();
        cursor = next_cursor;

        if cursor.is_none() {
            break;
        }
    }
    let metadata_index = Arc::new(metadata_index);

    let contents_controller = ContentController::new(
        ListContentsUseCase::new(metadata_index.clone()),
        GetContentUseCase::new(metadata_index.clone()),
    );
    let api_v0_router = Router::new().nest("/contents", contents_controller.router());
    let app = Router::new().nest("/api/v0", api_v0_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

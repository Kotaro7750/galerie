use std::sync::Arc;

use axum::Router;
use config::Config;
use controller::ContentController;
use serde::Deserialize;
use usecase::GetContentUseCase;

use crate::infrastructure::FileSystemContentStorage;
use crate::usecase::ListContentsUseCase;

mod controller;
mod domain;
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

    let content_storage = Arc::new(
        FileSystemContentStorage::new(
            config.content_storage.file_system.content_root_path,
            config.content_storage.file_system.content_url_base,
            config.content_storage.file_system.thumbnail_url_base,
        )
        .unwrap(),
    );
    let contents_controller = ContentController::new(
        ListContentsUseCase::new(content_storage.clone()),
        GetContentUseCase::new(content_storage.clone()),
    );
    let api_v0_router = Router::new().nest("/contents", contents_controller.router());
    let app = Router::new().nest("/api/v0", api_v0_router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct GalerieConfig {
    content_storage: ContentStorageConfig,
}

#[derive(Debug, Deserialize)]
struct ContentStorageConfig {
    file_system: FileSystemContentStorageConfig,
}

#[derive(Debug, Deserialize)]
struct FileSystemContentStorageConfig {
    content_root_path: String,
    content_url_base: String,
    thumbnail_url_base: String,
}

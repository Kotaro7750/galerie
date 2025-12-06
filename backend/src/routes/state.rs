use std::{sync::Arc, time::Instant};

use tokio::sync::RwLock;

use crate::{
    cache::{CacheSnapshot, CacheStore},
    config::AppConfig,
};

/// Shared application state cloned into each request handler.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub cache_store: Arc<CacheStore>,
    pub snapshot: Arc<RwLock<CacheSnapshot>>,
    pub boot_instant: Instant,
}

impl AppState {
    pub fn new(
        config: Arc<AppConfig>,
        cache_store: Arc<CacheStore>,
        snapshot: Arc<RwLock<CacheSnapshot>>,
    ) -> Self {
        Self {
            config,
            cache_store,
            snapshot,
            boot_instant: Instant::now(),
        }
    }
}

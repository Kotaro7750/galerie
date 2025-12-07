use std::{sync::Arc, time::Instant};

use crate::{cache::Cache, config::AppConfig};

/// Shared application state cloned into each request handler.
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub cache: Arc<Cache>,
    pub boot_instant: Instant,
}

impl AppState {
    pub fn new(config: Arc<AppConfig>, cache: Arc<Cache>) -> Self {
        Self {
            config,
            cache,
            boot_instant: Instant::now(),
        }
    }
}

pub mod scan;
pub mod store;

use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use tokio::{
    sync::{RwLock, RwLockReadGuard},
    time,
};

pub use store::{CacheSnapshot, CacheStore};

use crate::cache::scan::scan_media;

#[derive(Debug, Clone)]
/// Manages in-memory cache, persisted cache and scanning logic.
pub struct Cache {
    media_root: PathBuf,
    store: Arc<CacheStore>,
    // For performance, avoid accessing perssited cache every time
    snapshot: Arc<RwLock<CacheSnapshot>>,
}

impl<'a> Cache {
    pub fn new(media_root: impl Into<PathBuf>, cache_dir: impl Into<PathBuf>) -> Result<Self> {
        let store = CacheStore::new(cache_dir);

        let media_root: PathBuf = media_root.into();
        let snapshot = store.load_or_rebuild(|| scan_media(media_root.as_ref()))?;

        Ok(Self {
            media_root: media_root,
            store: Arc::new(store),
            snapshot: Arc::new(RwLock::new(snapshot)),
        })
    }

    pub fn new_with_snapshot(
        media_root: impl Into<PathBuf>,
        cache_dir: impl Into<PathBuf>,
        snapshot: CacheSnapshot,
    ) -> Self {
        let store = CacheStore::new(cache_dir);

        Self {
            media_root: media_root.into(),
            store: Arc::new(store),
            snapshot: Arc::new(RwLock::new(snapshot)),
        }
    }

    pub fn launch_sync_loop(&self, polling_interval: time::Duration) -> impl FnOnce() {
        let mut interval = time::interval(polling_interval);

        let media_root = self.media_root.clone();
        let store = self.store.clone();
        let snapshot = self.snapshot.clone();

        let handle = tokio::task::spawn(async move {
            loop {
                interval.tick().await;

                if let Err(err) =
                    Self::sync(media_root.clone(), store.clone(), snapshot.clone()).await
                {
                    tracing::error!(error = %err, "automatic index rebuild failed");
                } else {
                    tracing::info!("automatic index rebuild completed");
                }
            }
        });

        move || {
            handle.abort();
        }
    }

    pub async fn read_snapshot(&'a self) -> RwLockReadGuard<'a, CacheSnapshot> {
        self.snapshot.read().await
    }

    pub async fn trigger_rebuild(&self) -> Result<()> {
        let media_root = self.media_root.clone();
        let store = self.store.clone();
        let snapshot = self.snapshot.clone();

        tokio::task::spawn(async move {
            if let Err(err) = Cache::sync(media_root, store, snapshot).await {
                tracing::error!(error = %err, "manual index rebuild failed");
            } else {
                tracing::info!("manual index rebuild completed");
            }
        });

        Ok(())
    }

    async fn sync(
        media_root: impl Into<PathBuf>,
        store: Arc<CacheStore>,
        snapshot: Arc<RwLock<CacheSnapshot>>,
    ) -> Result<()> {
        let media_root_path: PathBuf = media_root.into();
        let media_files =
            tokio::task::spawn_blocking(move || scan_media(media_root_path.as_path())).await??;

        *snapshot.write().await = store.persist(media_files)?;

        Result::<(), anyhow::Error>::Ok(())
    }
}

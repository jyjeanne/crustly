//! Application Module
//!
//! Manages application lifecycle and state, including the background
//! file-system watcher that keeps the codebase symbol index up-to-date.

use anyhow::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self { running: true })
    }

    pub async fn run(&mut self) -> Result<()> {
        tracing::info!("Application starting...");
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Failed to create App")
    }
}

/// Spawn a background task that watches `project_root` for `.rs` file changes
/// and re-indexes them via `CodebaseIndex::index_file`.
///
/// Uses a `tokio::sync::mpsc` channel bridged from the synchronous notify watcher.
/// The task exits when the channel sender is dropped (i.e., the watcher drops).
pub fn start_file_watcher(pool: SqlitePool, project_root: PathBuf) {
    let root = Arc::new(project_root);
    let root_clone = root.clone();

    // Bridge: sync notify events → async tokio channel
    let (tx, mut rx) = tokio::sync::mpsc::channel::<notify::Result<Event>>(256);

    // Spawn the synchronous watcher on a dedicated OS thread (not tokio)
    std::thread::spawn(move || {
        let tx_inner = tx.clone();
        let mut watcher = match notify::recommended_watcher(move |res| {
            let _ = tx_inner.blocking_send(res);
        }) {
            Ok(w) => w,
            Err(e) => {
                tracing::error!("Failed to create file watcher: {}", e);
                return;
            }
        };

        if let Err(e) = watcher.watch(root_clone.as_path(), RecursiveMode::Recursive) {
            tracing::error!("Failed to watch {}: {}", root_clone.display(), e);
            return;
        }

        // Keep the watcher alive until the sender is dropped
        let _ = tx.blocking_send(Ok(Event::new(EventKind::Other)));
        loop {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    });

    // Async consumer of watcher events
    let root_async = root;
    tokio::spawn(async move {
        let index = crate::llm::agent::memory::CodebaseIndex::new(pool);
        while let Some(result) = rx.recv().await {
            let event = match result {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!("File watcher error: {}", e);
                    continue;
                }
            };

            let is_create_or_modify =
                matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_));
            if !is_create_or_modify {
                continue;
            }

            for path in &event.paths {
                if path.extension().is_some_and(|ext| ext == "rs")
                    && path.starts_with(root_async.as_path())
                {
                    tracing::debug!("Re-indexing: {}", path.display());
                    if let Err(e) = index.index_file(path).await {
                        tracing::warn!("Failed to index {}: {}", path.display(), e);
                    }
                }
            }
        }
    });
}

/// Check if path is a Rust source file inside the given root.
pub fn is_rust_file_in_root(path: &Path, root: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs") && path.starts_with(root)
}

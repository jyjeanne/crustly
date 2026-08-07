//! Backend glue for the TUI's "Local Models" dialog (Ctrl+G in Chat mode) -
//! the `llama.cpp` equivalent of `ollama_download.rs`'s Ctrl+D dialog.
//!
//! Unlike Ollama, there is no server and no online search API: this dialog
//! lists `.gguf` files already present under `providers.llama_cpp.models_dir`
//! (`llama_cpp_models::list_local_models`) and lets the user either switch to
//! one of them or type a direct URL / `hf:org/repo/file.gguf` shorthand to
//! download a new one (`llama_cpp_models::download_model`).
//!
//! Switching to a local model is not instant the way Ollama's Ctrl+W swap
//! is: `LlamaCppProvider::new()` blocks while it loads the whole model file
//! and spawns its worker thread (`llama-cpp-2-integration-plan.md` §4.5), so
//! it always runs inside `tokio::task::spawn_blocking` here, off the async
//! runtime, while the dialog shows a "Loading model…" state.
//!
//! Every function is compiled unconditionally so `AppMode::LlamaCppModelPicker`
//! always exists and always says something sensible; the real implementations
//! are split across two build-time gates
//! (`ccguf-managment-imrpoment-plan.md` Phase M0). `list_local`/
//! `spawn_download`/`spawn_delete` (pure filesystem/HTTP, no FFI) only need
//! `--features gguf-management` (on by `default`); `build_llama_cpp_provider`/
//! `spawn_switch` (loads model weights via `LlamaCppProvider::new()`) still
//! need the heavier `--features llama-cpp`. Each not-compiled-in fallback
//! degrades to "nothing found" / a clear "rebuild with..." error naming the
//! specific feature it needs, mirroring `ollama_download.rs`'s not-compiled-in
//! fallbacks.

use super::events::TuiEvent;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

/// A locally-present `.gguf` file, decoupled from the `llama-cpp` feature so
/// it can live on the always-compiled `TuiEvent`/`App` state - the TUI
/// equivalent of `llama_cpp_models::LocalGgufModel`. Field-for-field mirror
/// (see `list_local`'s mapping) so header-parsed metadata added in
/// `LocalGgufModel` (Phase M1) isn't dropped at this boundary even before a
/// later phase renders it.
#[derive(Debug, Clone)]
pub struct LlamaCppModelSummary {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub quantization_hint: Option<String>,
    pub architecture: Option<String>,
    pub parameter_count: Option<u64>,
    pub context_length: Option<u64>,
    pub has_chat_template: bool,
    pub display_name: Option<String>,
    pub estimated_memory_bytes: Option<u64>,
    pub estimated_memory_includes_kv_cache: bool,
    pub is_mmproj: bool,
    pub mmproj_path: Option<PathBuf>,
}

/// One progress update from an in-flight download, decoupled from the
/// `llama-cpp` feature for the same reason as `LlamaCppModelSummary`.
#[derive(Debug, Clone, Default)]
pub struct LlamaCppDownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
}

impl LlamaCppDownloadProgress {
    /// Completion fraction (0.0-1.0), if the server reported Content-Length.
    pub fn fraction(&self) -> Option<f64> {
        let total = self.total_bytes.filter(|t| *t > 0)? as f64;
        Some((self.bytes_downloaded as f64 / total).clamp(0.0, 1.0))
    }
}

/// GPU offload / quantization details for the Model Info panel (Ctrl+O)
/// when the active provider is `llama-cpp`. Context size is already shown
/// generically via `App::provider_context_window()` - not duplicated here.
#[derive(Debug, Clone)]
pub struct LlamaCppModelDetails {
    pub n_gpu_layers: u32,
    pub quantization_hint: Option<String>,
}

/// Shared slot the background switch task drops a freshly-built provider
/// into. Not carried on `TuiEvent` itself: `Provider` trait objects aren't
/// `Clone`/`Debug`, which `TuiEvent` derives - the event only signals "check
/// the slot", the provider crosses the thread boundary through this instead.
pub type PendingProvider = Arc<Mutex<Option<Arc<dyn crate::llm::provider::Provider>>>>;

/// List `.gguf` files from `models_dir`, `extra_model_paths`, and (when
/// `Some`) Ollama's manifest tree - see
/// `llama_cpp_models::list_all_local_models` for the merge/dedup behavior.
/// Returns an empty list (never an error) when the feature isn't compiled
/// in or nothing could be scanned - the dialog just shows "no local
/// models" rather than surfacing a scan failure the user can't act on.
#[cfg(feature = "gguf-management")]
pub async fn list_local(
    models_dir: PathBuf,
    extra_model_paths: Vec<PathBuf>,
    ollama_models_dir: Option<PathBuf>,
) -> Vec<LlamaCppModelSummary> {
    use crate::llm::provider::llama_cpp_models;

    llama_cpp_models::list_all_local_models(
        &models_dir,
        &extra_model_paths,
        ollama_models_dir.as_deref(),
    )
    .unwrap_or_default()
    .into_iter()
    .map(|m| LlamaCppModelSummary {
        path: m.path,
        size_bytes: m.size_bytes,
        quantization_hint: m.quantization_hint,
        architecture: m.architecture,
        parameter_count: m.parameter_count,
        context_length: m.context_length,
        has_chat_template: m.has_chat_template,
        display_name: m.display_name,
        estimated_memory_bytes: m.estimated_memory_bytes,
        estimated_memory_includes_kv_cache: m.estimated_memory_includes_kv_cache,
        is_mmproj: m.is_mmproj,
        mmproj_path: m.mmproj_path,
    })
    .collect()
}

#[cfg(not(feature = "gguf-management"))]
pub async fn list_local(
    _models_dir: PathBuf,
    _extra_model_paths: Vec<PathBuf>,
    _ollama_models_dir: Option<PathBuf>,
) -> Vec<LlamaCppModelSummary> {
    Vec::new()
}

/// Build an `LlamaCppProvider` for `model_path`, applying `config` (the
/// user's `[providers.llama_cpp]` section) for every setting except
/// `model_path` itself, which is always overridden to the picked file -
/// mirrors `ollama_download::build_ollama_provider`'s config-preservation
/// rationale. Blocking: loads the whole model file and spawns its worker
/// thread - callers must run this via `tokio::task::spawn_blocking`, never
/// directly on an async task.
#[cfg(feature = "llama-cpp")]
pub fn build_llama_cpp_provider(
    model_path: PathBuf,
    config: Option<&crate::config::LlamaCppProviderConfig>,
) -> Result<Arc<dyn crate::llm::provider::Provider>, String> {
    let mut cfg = config.cloned().unwrap_or_default();
    cfg.model_path = model_path;
    crate::llm::provider::LlamaCppProvider::new(&cfg)
        .map(|p| Arc::new(p) as Arc<dyn crate::llm::provider::Provider>)
        .map_err(|e| e.to_string())
}

#[cfg(not(feature = "llama-cpp"))]
pub fn build_llama_cpp_provider(
    _model_path: PathBuf,
    _config: Option<&crate::config::LlamaCppProviderConfig>,
) -> Result<Arc<dyn crate::llm::provider::Provider>, String> {
    Err(
        "This build of crustly was compiled without the 'llama-cpp' feature. \
         Rebuild with `--features llama-cpp`."
            .to_string(),
    )
}

/// Start loading `model_path` as the active provider in the background.
/// Runs the blocking model load via `spawn_blocking`; on success the built
/// provider is left in `slot` for the caller to pick up (see
/// `PendingProvider`'s doc comment for why it doesn't ride on `TuiEvent`),
/// and `TuiEvent::LlamaCppSwitchFinished` signals completion either way.
pub async fn spawn_switch(
    model_path: PathBuf,
    config: Option<crate::config::LlamaCppProviderConfig>,
    slot: PendingProvider,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let path_for_event = model_path.clone();
        let result = tokio::task::spawn_blocking(move || {
            build_llama_cpp_provider(model_path, config.as_ref())
        })
        .await;

        let error = match result {
            Ok(Ok(provider)) => {
                *slot.lock().unwrap_or_else(|e| e.into_inner()) = Some(provider);
                None
            }
            Ok(Err(e)) => Some(e),
            Err(join_err) => Some(format!("Model load task panicked: {join_err}")),
        };

        let _ = event_sender.send(TuiEvent::LlamaCppSwitchFinished {
            model_path: path_for_event,
            error,
        });
    })
}

/// Start downloading `source` (a direct URL or `hf:org/repo/file.gguf`
/// shorthand) into `models_dir` in the background, forwarding progress and
/// the final result through `event_sender`.
#[cfg(feature = "gguf-management")]
pub async fn spawn_download(
    source: String,
    models_dir: PathBuf,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    use crate::llm::provider::llama_cpp_models;

    tokio::spawn(async move {
        let (url, expected_sha256) = match llama_cpp_models::resolve_download_source(&source).await
        {
            Ok(resolved) => resolved,
            Err(e) => {
                let _ = event_sender.send(TuiEvent::LlamaCppDownloadFinished {
                    source,
                    error: Some(e.to_string()),
                });
                return;
            }
        };

        let (progress_tx, mut progress_rx) =
            tokio::sync::mpsc::unbounded_channel::<llama_cpp_models::DownloadProgress>();
        let forward_sender = event_sender.clone();
        tokio::spawn(async move {
            while let Some(p) = progress_rx.recv().await {
                let _ = forward_sender.send(TuiEvent::LlamaCppDownloadProgress(
                    LlamaCppDownloadProgress {
                        bytes_downloaded: p.bytes_downloaded,
                        total_bytes: p.total_bytes,
                    },
                ));
            }
        });

        let result = llama_cpp_models::download_model(
            &url,
            &models_dir,
            expected_sha256.as_deref(),
            progress_tx,
        )
        .await;
        let _ = event_sender.send(TuiEvent::LlamaCppDownloadFinished {
            source,
            error: result.err().map(|e| e.to_string()),
        });
    })
}

#[cfg(not(feature = "gguf-management"))]
pub async fn spawn_download(
    source: String,
    _models_dir: PathBuf,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let _ = event_sender.send(TuiEvent::LlamaCppDownloadFinished {
            source,
            error: Some(
                "This build of crustly was compiled without the 'gguf-management' feature. \
                 Rebuild with `--features gguf-management`."
                    .to_string(),
            ),
        });
    })
}

/// Start deleting the `.gguf` file at `path` in the background, forwarding
/// the result through `event_sender`.
#[cfg(feature = "gguf-management")]
pub async fn spawn_delete(
    path: PathBuf,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    use crate::llm::provider::llama_cpp_models;

    tokio::spawn(async move {
        let result = llama_cpp_models::delete_model(&path);
        let _ = event_sender.send(TuiEvent::LlamaCppDeleteFinished {
            path,
            error: result.err().map(|e| e.to_string()),
        });
    })
}

#[cfg(not(feature = "gguf-management"))]
pub async fn spawn_delete(
    path: PathBuf,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let _ = event_sender.send(TuiEvent::LlamaCppDeleteFinished {
            path,
            error: Some(
                "This build of crustly was compiled without the 'gguf-management' feature. \
                 Rebuild with `--features gguf-management`."
                    .to_string(),
            ),
        });
    })
}

/// Filter `models` to those whose file name contains `query`
/// (case-insensitive substring). Empty query returns every model
/// unfiltered - same shape as `ollama_download::filter_suggestions`, minus
/// the curated-list/installed-merge logic Ollama needs and this dialog
/// doesn't (there's only ever one source: the local directory scan).
pub fn filter_local(models: &[LlamaCppModelSummary], query: &str) -> Vec<LlamaCppModelSummary> {
    let query_lc = query.trim().to_lowercase();
    if query_lc.is_empty() {
        return models.to_vec();
    }
    models
        .iter()
        .filter(|m| {
            m.path
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase().contains(&query_lc))
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(name: &str) -> LlamaCppModelSummary {
        LlamaCppModelSummary {
            path: PathBuf::from(name),
            size_bytes: 1_000_000,
            quantization_hint: Some("Q4_K_M".to_string()),
            architecture: None,
            parameter_count: None,
            context_length: None,
            has_chat_template: false,
            display_name: None,
            estimated_memory_bytes: None,
            estimated_memory_includes_kv_cache: false,
            is_mmproj: false,
            mmproj_path: None,
        }
    }

    #[test]
    fn download_progress_fraction() {
        let p = LlamaCppDownloadProgress {
            bytes_downloaded: 50,
            total_bytes: Some(200),
        };
        assert_eq!(p.fraction(), Some(0.25));
    }

    #[test]
    fn download_progress_fraction_unknown_total_is_none() {
        let p = LlamaCppDownloadProgress {
            bytes_downloaded: 50,
            total_bytes: None,
        };
        assert_eq!(p.fraction(), None);
    }

    #[test]
    fn filter_local_empty_query_returns_all() {
        let models = vec![model("qwen.gguf"), model("llama.gguf")];
        assert_eq!(filter_local(&models, "").len(), 2);
    }

    #[test]
    fn filter_local_matches_substring_case_insensitive() {
        let models = vec![model("Qwen2.5-Coder-7B-Q4_K_M.gguf"), model("llama3.gguf")];
        let filtered = filter_local(&models, "qwen");
        assert_eq!(filtered.len(), 1);
        assert_eq!(
            filtered[0].path,
            PathBuf::from("Qwen2.5-Coder-7B-Q4_K_M.gguf")
        );
    }

    #[test]
    fn filter_local_no_match_returns_empty() {
        let models = vec![model("qwen.gguf")];
        assert!(filter_local(&models, "mistral").is_empty());
    }

    #[cfg(not(feature = "gguf-management"))]
    #[tokio::test]
    async fn list_local_without_feature_is_empty() {
        assert!(list_local(PathBuf::from("/tmp/anything"), Vec::new(), None)
            .await
            .is_empty());
    }

    #[cfg(not(feature = "llama-cpp"))]
    #[tokio::test]
    async fn build_provider_without_feature_reports_a_clear_error() {
        let err = match build_llama_cpp_provider(PathBuf::from("/tmp/model.gguf"), None) {
            Err(e) => e,
            Ok(_) => panic!("expected an error without the llama-cpp feature"),
        };
        assert!(
            err.contains("llama-cpp"),
            "error should name the feature: {err}"
        );
    }
}

//! Backend glue for the TUI's "Model Download" dialog (Ctrl+D in Chat mode).
//!
//! Lets the user type/pick an Ollama model name and pull it without leaving
//! Crustly. Ollama has no API to *search* its online library - only to pull
//! a model whose `repo:tag` name is already known (like `ollama pull <name>`
//! on the CLI) - so suggestions here are a curated static list plus whatever
//! is already installed locally, not a text search over a remote catalog.
//!
//! The actual pull only runs when this crate is built with `--features
//! ollama`; otherwise `spawn_pull` immediately reports a clear error instead
//! of silently doing nothing.

use super::events::TuiEvent;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

/// Curated list of models known to work well for coding, mirroring the
/// "Recommended Local Models" section in the README. Purely a starting
/// point for the suggestions list - the user can type any other name.
pub const CURATED_MODELS: &[&str] = &[
    "qwen2.5-coder:7b",
    "gemma3:12b",
    "gemma4:26b",
    "llama3.1:8b",
    "llama3.2:3b",
    "mistral:latest",
    "deepseek-r1:14b",
];

/// One progress update from an in-flight pull, decoupled from the `ollama`
/// feature so it can live on the always-compiled `TuiEvent` enum.
#[derive(Debug, Clone)]
pub struct ModelPullProgress {
    pub status: String,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}

impl ModelPullProgress {
    /// Completion fraction for the current layer (0.0-1.0), if known.
    pub fn fraction(&self) -> Option<f64> {
        let total = self.total.filter(|t| *t > 0)? as f64;
        let completed = self.completed? as f64;
        Some((completed / total).clamp(0.0, 1.0))
    }
}

/// Build the suggestion list for the current query: locally-installed
/// models first (already downloaded, marked separately by the caller),
/// then curated picks, filtered by a case-insensitive substring match.
/// Empty query returns the unfiltered combined list.
pub fn filter_suggestions(query: &str, installed: &[String]) -> Vec<String> {
    let query_lc = query.trim().to_lowercase();

    let mut seen = std::collections::HashSet::new();
    let mut suggestions = Vec::new();

    for name in installed
        .iter()
        .cloned()
        .chain(CURATED_MODELS.iter().map(|s| s.to_string()))
    {
        if !query_lc.is_empty() && !name.to_lowercase().contains(&query_lc) {
            continue;
        }
        if seen.insert(name.clone()) {
            suggestions.push(name);
        }
    }

    suggestions
}

/// Fetch the list of locally-installed model names. Returns an empty list
/// (rather than erroring) when Ollama is unreachable or the feature isn't
/// compiled in - the suggestions list still works from the curated list.
#[cfg(feature = "ollama")]
pub async fn fetch_installed_models(host: String) -> Vec<String> {
    crate::llm::provider::ollama_models::list_models(&host)
        .await
        .map(|models| models.into_iter().map(|m| m.name).collect())
        .unwrap_or_default()
}

#[cfg(not(feature = "ollama"))]
pub async fn fetch_installed_models(_host: String) -> Vec<String> {
    Vec::new()
}

/// Build a native Ollama provider for `model`, for the Provider Switch
/// dialog (Ctrl+W). Returns a clear error instead of silently doing
/// nothing when this build wasn't compiled with `--features ollama`.
#[cfg(feature = "ollama")]
pub fn build_ollama_provider(
    host: &str,
    model: &str,
) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String> {
    Ok(std::sync::Arc::new(
        crate::llm::provider::OllamaProvider::new(host.to_string())
            .with_default_model(model.to_string()),
    ))
}

#[cfg(not(feature = "ollama"))]
pub fn build_ollama_provider(
    _host: &str,
    _model: &str,
) -> Result<std::sync::Arc<dyn crate::llm::provider::Provider>, String> {
    Err(
        "This build of crustly was compiled without the 'ollama' feature. \
         Rebuild with `--features ollama` (or `all-llm`)."
            .to_string(),
    )
}

/// Start pulling `model` in the background, forwarding progress and the
/// final result through `event_sender`. Returns the `JoinHandle` of the
/// pull task itself so the caller can `.abort()` it (e.g. on Esc).
#[cfg(feature = "ollama")]
pub async fn spawn_pull(
    host: String,
    model: String,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    use crate::llm::provider::ollama_models;

    let (progress_tx, mut progress_rx) =
        tokio::sync::mpsc::unbounded_channel::<ollama_models::PullProgress>();

    let forward_sender = event_sender.clone();
    tokio::spawn(async move {
        while let Some(p) = progress_rx.recv().await {
            let _ = forward_sender.send(TuiEvent::OllamaPullProgress(ModelPullProgress {
                status: p.status,
                total: p.total,
                completed: p.completed,
            }));
        }
    });

    let model_for_task = model.clone();
    tokio::spawn(async move {
        let result = ollama_models::pull_model(&host, &model_for_task, progress_tx).await;
        let _ = event_sender.send(TuiEvent::OllamaPullFinished {
            model: model_for_task,
            error: result.err().map(|e| e.to_string()),
        });
    })
}

#[cfg(not(feature = "ollama"))]
pub async fn spawn_pull(
    _host: String,
    model: String,
    event_sender: UnboundedSender<TuiEvent>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let _ = event_sender.send(TuiEvent::OllamaPullFinished {
            model,
            error: Some(
                "This build of crustly was compiled without the 'ollama' feature. \
                 Rebuild with `--features ollama` (or `all-llm`)."
                    .to_string(),
            ),
        });
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_suggestions_empty_query_returns_all_deduped() {
        let installed = vec!["qwen2.5-coder:7b".to_string(), "llava:13b".to_string()];
        let suggestions = filter_suggestions("", &installed);
        // installed models first, deduplicated against curated overlap
        assert_eq!(suggestions[0], "qwen2.5-coder:7b");
        assert_eq!(suggestions[1], "llava:13b");
        assert!(
            suggestions
                .iter()
                .filter(|s| *s == "qwen2.5-coder:7b")
                .count()
                == 1
        );
    }

    #[test]
    fn filter_suggestions_matches_substring_case_insensitive() {
        let suggestions = filter_suggestions("LLAMA", &[]);
        assert!(suggestions.contains(&"llama3.1:8b".to_string()));
        assert!(suggestions.contains(&"llama3.2:3b".to_string()));
        assert!(!suggestions.contains(&"mistral:latest".to_string()));
    }

    #[test]
    fn pull_progress_fraction() {
        let p = ModelPullProgress {
            status: "pulling abc".to_string(),
            total: Some(200),
            completed: Some(50),
        };
        assert_eq!(p.fraction(), Some(0.25));
    }
}

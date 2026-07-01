//! Ollama model management (list/pull/delete/show).
//!
//! Thin wrapper around `ollama-rs`'s `/api/tags`, `/api/pull`, `/api/delete`
//! and `/api/show` endpoints, decoupled from `ollama-rs`'s own types so
//! callers (CLI, TUI) depend on a small, stable surface.
//!
//! Note: Ollama has no API to *search* its online model library - only to
//! pull a model whose `repo:tag` name is already known (exactly like
//! `ollama pull <name>` on the CLI). Callers are responsible for supplying
//! (or letting the user type/pick) a valid name.

use anyhow::{Context, Result};
use futures::StreamExt as _;
use ollama_rs::generation::embeddings::request::{EmbeddingsInput, GenerateEmbeddingsRequest};
use ollama_rs::Ollama;
use tokio::sync::mpsc::UnboundedSender;

/// A locally-installed model, as reported by `/api/tags`.
#[derive(Debug, Clone)]
pub struct LocalModelInfo {
    pub name: String,
    pub size_bytes: u64,
    pub modified_at: String,
}

/// One progress update from an in-flight `pull_model` download.
#[derive(Debug, Clone)]
pub struct PullProgress {
    /// Human-readable status, e.g. "pulling manifest", "pulling <digest>",
    /// "verifying sha256 digest", "success".
    pub status: String,
    /// Digest of the layer currently being downloaded, if applicable.
    pub digest: Option<String>,
    /// Total bytes for the current layer.
    pub total: Option<u64>,
    /// Bytes downloaded so far for the current layer.
    pub completed: Option<u64>,
}

impl PullProgress {
    /// Whether this update indicates the pull has finished successfully.
    pub fn is_success(&self) -> bool {
        self.status.eq_ignore_ascii_case("success")
    }

    /// Completion fraction for the current layer (0.0-1.0), if known.
    pub fn fraction(&self) -> Option<f64> {
        let total = self.total.filter(|t| *t > 0)? as f64;
        let completed = self.completed? as f64;
        Some((completed / total).clamp(0.0, 1.0))
    }
}

/// Details about a model, as reported by `/api/show`.
#[derive(Debug, Clone)]
pub struct ModelDetails {
    pub license: String,
    pub parameters: String,
    pub template: String,
    pub capabilities: Vec<String>,
}

fn client_for(host: &str) -> Result<Ollama> {
    Ollama::try_new(host).with_context(|| format!("Invalid Ollama host: {host}"))
}

/// List models already pulled/installed on the target Ollama instance.
pub async fn list_models(host: &str) -> Result<Vec<LocalModelInfo>> {
    let client = client_for(host)?;
    let models = client
        .list_local_models()
        .await
        .context("Failed to list local Ollama models")?;

    Ok(models
        .into_iter()
        .map(|m| LocalModelInfo {
            name: m.name,
            size_bytes: m.size,
            modified_at: m.modified_at,
        })
        .collect())
}

/// Show details about a model (license, parameters, template, capabilities).
pub async fn show_model(host: &str, model_name: &str) -> Result<ModelDetails> {
    let client = client_for(host)?;
    let info = client
        .show_model_info(model_name.to_string())
        .await
        .with_context(|| format!("Failed to show info for model '{model_name}'"))?;

    Ok(ModelDetails {
        license: info.license,
        parameters: info.parameters,
        template: info.template,
        capabilities: info.capabilities,
    })
}

/// Delete a locally-installed model.
pub async fn delete_model(host: &str, model_name: &str) -> Result<()> {
    let client = client_for(host)?;
    client
        .delete_model(model_name.to_string())
        .await
        .with_context(|| format!("Failed to delete model '{model_name}'"))?;
    Ok(())
}

/// Pull (download) a model, streaming progress updates through `progress_tx`.
///
/// Used by both the `crustly ollama pull` CLI command and the TUI's
/// "Model Download" dialog - callers own how progress is consumed (printed
/// to stdout, or rendered as a live progress bar).
pub async fn pull_model(
    host: &str,
    model_name: &str,
    progress_tx: UnboundedSender<PullProgress>,
) -> Result<()> {
    let client = client_for(host)?;
    let mut stream = client
        .pull_model_stream(model_name.to_string(), false)
        .await
        .with_context(|| format!("Failed to start pulling model '{model_name}'"))?;

    while let Some(item) = stream.next().await {
        let status = item.with_context(|| format!("Error while pulling model '{model_name}'"))?;
        // Ignore send errors: the receiver (CLI printer / TUI dialog) may
        // have been dropped if the user cancelled or the view closed.
        let _ = progress_tx.send(PullProgress {
            status: status.message,
            digest: status.digest,
            total: status.total,
            completed: status.completed,
        });
    }

    Ok(())
}

/// Generate embedding vectors for one or more input strings using an
/// embedding-capable Ollama model (e.g. `nomic-embed-text`, `mxbai-embed-large`).
///
/// Not wired into a RAG/retrieval layer - Crustly doesn't have one yet. This
/// exposes the raw capability (one embedding vector per input string, same
/// order) for future callers (semantic search, codebase indexing, etc.).
pub async fn generate_embeddings(
    host: &str,
    model_name: &str,
    input: Vec<String>,
) -> Result<Vec<Vec<f32>>> {
    let client = client_for(host)?;
    let request =
        GenerateEmbeddingsRequest::new(model_name.to_string(), EmbeddingsInput::Multiple(input));

    let response = client
        .generate_embeddings(request)
        .await
        .with_context(|| format!("Failed to generate embeddings with model '{model_name}'"))?;

    Ok(response.embeddings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_progress_fraction() {
        let p = PullProgress {
            status: "pulling abc123".to_string(),
            digest: Some("abc123".to_string()),
            total: Some(1000),
            completed: Some(250),
        };
        assert_eq!(p.fraction(), Some(0.25));
    }

    #[test]
    fn pull_progress_fraction_missing_data() {
        let p = PullProgress {
            status: "pulling manifest".to_string(),
            digest: None,
            total: None,
            completed: None,
        };
        assert_eq!(p.fraction(), None);
    }

    #[test]
    fn pull_progress_is_success() {
        let p = PullProgress {
            status: "success".to_string(),
            digest: None,
            total: None,
            completed: None,
        };
        assert!(p.is_success());
    }

    #[test]
    fn invalid_host_returns_error() {
        let result = client_for("not a url");
        assert!(result.is_err());
    }

    #[test]
    fn embeddings_request_serializes_model_and_input() {
        // Mirrors what `generate_embeddings()` builds internally - verifies
        // the wire shape without needing a live Ollama server.
        let request = GenerateEmbeddingsRequest::new(
            "nomic-embed-text".to_string(),
            EmbeddingsInput::Multiple(vec!["hello".to_string(), "world".to_string()]),
        );
        let value = serde_json::to_value(&request).expect("serialize request");
        assert_eq!(value["model"], "nomic-embed-text");
        assert_eq!(value["input"], serde_json::json!(["hello", "world"]));
    }

    #[test]
    fn embeddings_request_single_input_is_not_wrapped_in_array() {
        // EmbeddingsInput::Single serializes as a bare string, not a
        // one-element array - Ollama's API distinguishes the two shapes.
        let request = GenerateEmbeddingsRequest::new(
            "nomic-embed-text".to_string(),
            EmbeddingsInput::Single("hello".to_string()),
        );
        let value = serde_json::to_value(&request).expect("serialize request");
        assert_eq!(value["input"], serde_json::json!("hello"));
    }
}

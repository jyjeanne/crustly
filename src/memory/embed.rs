//! Embedding providers for semantic recall (docs/improve-memory.md §6.2,
//! §11.1 to §11.3; tasks P2.2 to P2.4).
//!
//! Two providers: Ollama (`/api/embed`) and any OpenAI-compatible server
//! (`/v1/embeddings`, which LM Studio, llama.cpp's server, vLLM and OpenAI
//! all speak). They use plain `reqwest`, not the SSRF-guarded client, because
//! they talk to localhost by design (D9).

use super::vector::normalize;
use async_trait::async_trait;
use serde::Deserialize;
use std::fmt;
use std::time::Duration;

/// Whether an embedding is for a search query or for a stored memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedPurpose {
    Query,
    Document,
}

/// Why an embedding could not be made.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbedError {
    /// No answer: connection refused, timeout, or an unreadable response.
    Transport(String),
    /// The server answered with an error status.
    Status { code: u16, body: String },
    /// The answer did not have the expected shape.
    BadResponse(String),
    /// Vectors of different sizes came back for one model.
    Dimensions { expected: usize, got: usize },
    /// The settings cannot work (for example an empty model name).
    Config(String),
}

impl fmt::Display for EmbedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(e) => write!(f, "embedding server unreachable: {e}"),
            Self::Status { code, body } => write!(f, "embedding server answered {code}: {body}"),
            Self::BadResponse(e) => write!(f, "embedding server sent an unexpected answer: {e}"),
            Self::Dimensions { expected, got } => {
                write!(
                    f,
                    "embedding model changed size: {expected} then {got} dimensions"
                )
            }
            Self::Config(e) => write!(f, "embedding settings: {e}"),
        }
    }
}

impl std::error::Error for EmbedError {}

/// Produces vectors for memory texts. `model_id` is stored with every vector,
/// so vectors from different models are never compared.
#[async_trait]
pub trait Embedder: Send + Sync {
    /// Stable identifier, e.g. `ollama:nomic-embed-text`.
    fn model_id(&self) -> &str;

    /// One unit-length vector per input, in order.
    async fn embed(
        &self,
        purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError>;
}

/// Task prefixes some embedding models need (§11.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prefixes {
    pub query: String,
    pub document: String,
}

impl Prefixes {
    /// The built-in prefixes for a known model, with any configured override
    /// taking precedence.
    pub fn for_model(
        model: &str,
        query_override: Option<&str>,
        document_override: Option<&str>,
    ) -> Self {
        let base = model.split(':').next().unwrap_or(model);
        let (query, document) = match base {
            "nomic-embed-text" => ("search_query: ", "search_document: "),
            "mxbai-embed-large" => (
                "Represent this sentence for searching relevant passages: ",
                "",
            ),
            _ => ("", ""),
        };
        Self {
            query: query_override.unwrap_or(query).to_string(),
            document: document_override.unwrap_or(document).to_string(),
        }
    }

    fn apply(&self, purpose: EmbedPurpose, text: &str) -> String {
        match purpose {
            EmbedPurpose::Query => format!("{}{text}", self.query),
            EmbedPurpose::Document => format!("{}{text}", self.document),
        }
    }
}

/// Settings shared by both providers.
#[derive(Debug, Clone)]
pub struct EmbedSettings {
    pub base_url: String,
    pub model: String,
    pub prefixes: Prefixes,
    pub timeout: Duration,
    /// Sent as a bearer token to OpenAI-compatible servers that need one.
    pub api_key: Option<String>,
}

fn check_settings(settings: &EmbedSettings) -> Result<(), EmbedError> {
    if settings.model.trim().is_empty() {
        return Err(EmbedError::Config("the model name is empty".to_string()));
    }
    if !(settings.base_url.starts_with("http://") || settings.base_url.starts_with("https://")) {
        return Err(EmbedError::Config(format!(
            "the base URL must start with http:// or https://, not {}",
            settings.base_url
        )));
    }
    Ok(())
}

fn client(timeout: Duration) -> Result<reqwest::Client, EmbedError> {
    reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| EmbedError::Config(e.to_string()))
}

/// Check the vectors have one size, normalise them, and return them.
fn finish(vectors: Vec<Vec<f32>>, expected_count: usize) -> Result<Vec<Vec<f32>>, EmbedError> {
    if vectors.len() != expected_count {
        return Err(EmbedError::BadResponse(format!(
            "asked for {expected_count} embeddings, got {}",
            vectors.len()
        )));
    }
    let Some(dims) = vectors.first().map(Vec::len) else {
        return Ok(Vec::new());
    };
    let mut out = Vec::with_capacity(vectors.len());
    for v in vectors {
        if v.len() != dims {
            return Err(EmbedError::Dimensions {
                expected: dims,
                got: v.len(),
            });
        }
        let unit = normalize(&v).map_err(|e| EmbedError::BadResponse(e.to_string()))?;
        out.push(unit);
    }
    Ok(out)
}

// ── Ollama (P2.3) ───────────────────────────────────────────────────────

#[derive(Deserialize)]
struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

/// Embeddings from a local Ollama server's `/api/embed`.
pub struct OllamaEmbedder {
    settings: EmbedSettings,
    model_id: String,
    http: reqwest::Client,
}

impl OllamaEmbedder {
    pub fn new(settings: EmbedSettings) -> Result<Self, EmbedError> {
        check_settings(&settings)?;
        let http = client(settings.timeout)?;
        let model_id = format!("ollama:{}", settings.model);
        Ok(Self {
            settings,
            model_id,
            http,
        })
    }
}

#[async_trait]
impl Embedder for OllamaEmbedder {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn embed(
        &self,
        purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let input: Vec<String> = texts
            .iter()
            .map(|t| self.settings.prefixes.apply(purpose, t))
            .collect();
        let url = format!("{}/api/embed", self.settings.base_url.trim_end_matches('/'));
        let response = self
            .http
            .post(&url)
            .json(&serde_json::json!({ "model": self.settings.model, "input": input }))
            .send()
            .await
            .map_err(|e| EmbedError::Transport(e.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(EmbedError::Status {
                code: status.as_u16(),
                body,
            });
        }
        let parsed: OllamaEmbedResponse = response
            .json()
            .await
            .map_err(|e| EmbedError::BadResponse(e.to_string()))?;
        finish(parsed.embeddings, texts.len())
    }
}

// ── OpenAI-compatible (P2.4) ────────────────────────────────────────────

#[derive(Deserialize)]
struct OpenAiEmbedResponse {
    data: Vec<OpenAiEmbedding>,
}

#[derive(Deserialize)]
struct OpenAiEmbedding {
    index: usize,
    embedding: Vec<f32>,
}

/// Embeddings from any server that speaks `/v1/embeddings`.
pub struct OpenAiCompatibleEmbedder {
    settings: EmbedSettings,
    model_id: String,
    http: reqwest::Client,
}

impl OpenAiCompatibleEmbedder {
    pub fn new(settings: EmbedSettings) -> Result<Self, EmbedError> {
        check_settings(&settings)?;
        let http = client(settings.timeout)?;
        let model_id = format!("openai:{}", settings.model);
        Ok(Self {
            settings,
            model_id,
            http,
        })
    }
}

#[async_trait]
impl Embedder for OpenAiCompatibleEmbedder {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn embed(
        &self,
        purpose: EmbedPurpose,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbedError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }
        let input: Vec<String> = texts
            .iter()
            .map(|t| self.settings.prefixes.apply(purpose, t))
            .collect();
        let url = format!(
            "{}/v1/embeddings",
            self.settings.base_url.trim_end_matches('/')
        );
        let mut request = self
            .http
            .post(&url)
            .json(&serde_json::json!({ "model": self.settings.model, "input": input }));
        if let Some(key) = &self.settings.api_key {
            request = request.bearer_auth(key);
        }
        let response = request
            .send()
            .await
            .map_err(|e| EmbedError::Transport(e.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(EmbedError::Status {
                code: status.as_u16(),
                body,
            });
        }
        let mut parsed: OpenAiEmbedResponse = response
            .json()
            .await
            .map_err(|e| EmbedError::BadResponse(e.to_string()))?;
        // The servers return items in any order; the index says which input.
        parsed.data.sort_by_key(|item| item.index);
        let vectors: Vec<Vec<f32>> = parsed.data.into_iter().map(|item| item.embedding).collect();
        finish(vectors, texts.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// Serve one HTTP response with `body`, and return the request the client
    /// sent (headers and body) through the returned receiver.
    async fn mock_server(
        status: &'static str,
        body: String,
    ) -> (String, tokio::sync::oneshot::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, rx) = tokio::sync::oneshot::channel();
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 16384];
            let n = socket.read(&mut buf).await.unwrap();
            let _ = tx.send(String::from_utf8_lossy(&buf[..n]).into_owned());
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(response.as_bytes()).await;
        });
        (format!("http://{addr}"), rx)
    }

    fn settings(base_url: String, model: &str) -> EmbedSettings {
        EmbedSettings {
            base_url,
            model: model.to_string(),
            prefixes: Prefixes::for_model(model, None, None),
            timeout: Duration::from_secs(5),
            api_key: None,
        }
    }

    #[test]
    fn nomic_model_gets_its_task_prefixes() {
        let p = Prefixes::for_model("nomic-embed-text", None, None);
        assert_eq!(p.query, "search_query: ");
        assert_eq!(p.document, "search_document: ");
        assert_eq!(Prefixes::for_model("unknown-model", None, None).query, "");
    }

    #[test]
    fn configured_prefixes_override_the_built_in_ones() {
        let p = Prefixes::for_model("nomic-embed-text", Some("Q: "), Some(""));
        assert_eq!(p.query, "Q: ");
        assert_eq!(p.document, "");
    }

    #[test]
    fn settings_are_checked_before_any_request() {
        let mut s = settings("http://localhost:11434".into(), "");
        assert!(OllamaEmbedder::new(s.clone()).is_err(), "empty model");
        s.model = "nomic-embed-text".into();
        s.base_url = "localhost:11434".into();
        assert!(OllamaEmbedder::new(s).is_err(), "no scheme");
    }

    #[tokio::test]
    async fn ollama_sends_prefixed_input_and_returns_unit_vectors() {
        let (url, request) = mock_server(
            "200 OK",
            r#"{"embeddings":[[3.0,4.0],[0.0,2.0]]}"#.to_string(),
        )
        .await;
        let embedder = OllamaEmbedder::new(settings(url, "nomic-embed-text")).unwrap();
        assert_eq!(embedder.model_id(), "ollama:nomic-embed-text");

        let vectors = embedder
            .embed(EmbedPurpose::Document, &["tokio".into(), "rust".into()])
            .await
            .unwrap();
        let sent = request.await.unwrap();
        assert!(sent.starts_with("POST /api/embed"), "{sent}");
        assert!(
            sent.contains("search_document: tokio"),
            "prefix sent: {sent}"
        );
        assert!((vectors[0][0] - 0.6).abs() < 1e-6, "{vectors:?}");
        assert!((vectors[1][1] - 1.0).abs() < 1e-6);
    }

    #[tokio::test]
    async fn ollama_reports_an_error_status_without_panicking() {
        let (url, _request) = mock_server("500 Internal Server Error", "boom".to_string()).await;
        let embedder = OllamaEmbedder::new(settings(url, "nomic-embed-text")).unwrap();
        let err = embedder
            .embed(EmbedPurpose::Query, &["x".into()])
            .await
            .unwrap_err();
        assert!(
            matches!(err, EmbedError::Status { code: 500, .. }),
            "{err:?}"
        );
    }

    #[tokio::test]
    async fn ollama_refuses_a_wrong_number_of_vectors() {
        let (url, _request) =
            mock_server("200 OK", r#"{"embeddings":[[1.0,0.0]]}"#.to_string()).await;
        let embedder = OllamaEmbedder::new(settings(url, "nomic-embed-text")).unwrap();
        let err = embedder
            .embed(EmbedPurpose::Document, &["a".into(), "b".into()])
            .await
            .unwrap_err();
        assert!(matches!(err, EmbedError::BadResponse(_)), "{err:?}");
    }

    #[tokio::test]
    async fn openai_orders_by_index_and_sends_the_key() {
        let body =
            r#"{"data":[{"index":1,"embedding":[0.0,2.0]},{"index":0,"embedding":[3.0,4.0]}]}"#;
        let (url, request) = mock_server("200 OK", body.to_string()).await;
        let mut s = settings(url, "text-embedding-3-small");
        s.api_key = Some("test-key".into());
        let embedder = OpenAiCompatibleEmbedder::new(s).unwrap();
        assert_eq!(embedder.model_id(), "openai:text-embedding-3-small");

        let vectors = embedder
            .embed(EmbedPurpose::Query, &["first".into(), "second".into()])
            .await
            .unwrap();
        let sent = request.await.unwrap();
        assert!(sent.starts_with("POST /v1/embeddings"), "{sent}");
        assert!(
            sent.to_lowercase()
                .contains("authorization: bearer test-key"),
            "{sent}"
        );
        assert!(
            (vectors[0][0] - 0.6).abs() < 1e-6,
            "index 0 comes first: {vectors:?}"
        );
    }

    #[tokio::test]
    async fn an_unreachable_server_is_a_transport_error() {
        // Port 1 on localhost is closed, so the connection is refused at once.
        let embedder =
            OllamaEmbedder::new(settings("http://127.0.0.1:1".into(), "nomic-embed-text")).unwrap();
        let err = embedder
            .embed(EmbedPurpose::Query, &["x".into()])
            .await
            .unwrap_err();
        assert!(matches!(err, EmbedError::Transport(_)), "{err:?}");
    }
}

//! `llama.cpp` model management (list/download/delete local `.gguf` files).
//!
//! Unlike `ollama_models.rs`, there is no server to address - this is local
//! filesystem scanning plus a plain HTTP download, named and shaped to
//! match `ollama_models.rs` field-for-field (`llama-cpp-2-integration-plan.md`
//! §8) rather than inventing new vocabulary for the same kind of operation.

use anyhow::{Context, Result};
use futures::StreamExt as _;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt as _;
use tokio::sync::mpsc::UnboundedSender;

/// A locally-present `.gguf` file - the llama.cpp equivalent of
/// `ollama_models::LocalModelInfo` (which reports installed models via
/// Ollama's `/api/tags` instead of a directory scan).
#[derive(Debug, Clone)]
pub struct LocalGgufModel {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified_at: String,
    /// Best-effort quantization label, layered: the GGUF header's
    /// `general.file_type` (precise, e.g. "Q4_K_M") when readable, else the
    /// header's per-tensor type mode (coarser, e.g. "Q4_K"), else a
    /// filename-convention guess (e.g. spotting "Q4_K_M" as a substring of
    /// the filename) when the header itself can't be read at all. `None`
    /// only if none of those three sources produced anything - displayed as
    /// "unknown" by callers rather than guessed further. See
    /// `crate::llm::provider::gguf_metadata` for the header parser.
    pub quantization_hint: Option<String>,
    /// GGUF `general.architecture` (e.g. "llama", "qwen2"). `None` if the
    /// header couldn't be read or didn't set it.
    pub architecture: Option<String>,
    /// Total scalar weight count, summed across the header's tensor-info
    /// dimensions - independent of quantization. `None` if the header
    /// couldn't be read.
    pub parameter_count: Option<u64>,
    /// The model's trained/native context window, read from the
    /// architecture-namespaced `*.context_length` GGUF key. `None` if the
    /// header couldn't be read or didn't set it.
    pub context_length: Option<u64>,
    /// Whether the header has a `tokenizer.chat_template` key. `false` (not
    /// `Option`) if the header couldn't be read, same as "no template
    /// found" - this field is advisory display data, not something callers
    /// branch safety-critical behavior on.
    pub has_chat_template: bool,
}

/// One progress update from an in-flight `download_model` transfer. The
/// llama.cpp equivalent of `ollama_models::PullProgress`, adapted to plain
/// HTTP byte progress (no layer/digest concept - a `.gguf` download is a
/// single file, not a multi-layer manifest pull).
#[derive(Debug, Clone, Default)]
pub struct DownloadProgress {
    pub bytes_downloaded: u64,
    pub total_bytes: Option<u64>,
}

impl DownloadProgress {
    /// Completion fraction (0.0-1.0), if the server reported Content-Length.
    /// Same clamp-and-None-on-unknown-total shape as
    /// `ollama_models::PullProgress::fraction()`.
    pub fn fraction(&self) -> Option<f64> {
        let total = self.total_bytes.filter(|t| *t > 0)? as f64;
        Some((self.bytes_downloaded as f64 / total).clamp(0.0, 1.0))
    }
}

/// Quantization tags `llama.cpp`/`llama.cpp`-ecosystem tools commonly embed
/// in `.gguf` filenames, checked case-insensitively. Ordered roughly from
/// most to least specific so a longer, more specific tag matches before a
/// shorter substring of it could (e.g. "Q4_K_M" before "Q4").
const QUANTIZATION_TAGS: &[&str] = &[
    "Q2_K", "Q3_K_S", "Q3_K_M", "Q3_K_L", "Q3_K", "Q4_0", "Q4_1", "Q4_K_S", "Q4_K_M", "Q4_K",
    "Q5_0", "Q5_1", "Q5_K_S", "Q5_K_M", "Q5_K", "Q6_K", "Q8_0", "IQ2_XXS", "IQ2_XS", "IQ2_S",
    "IQ2_M", "IQ3_XXS", "IQ3_S", "IQ3_M", "IQ4_XS", "IQ4_NL", "F16", "F32", "BF16",
];

/// Best-effort quantization guess from a `.gguf` filename's convention
/// (e.g. `Ornith-1.0-9B-Q4_K_M.gguf` -> `Some("Q4_K_M")`). Not a GGUF
/// header read - just the filename convention nearly every publisher
/// follows. `None` if nothing recognizable matches.
pub fn quantization_hint_from_filename(filename: &str) -> Option<String> {
    let upper = filename.to_uppercase();
    QUANTIZATION_TAGS
        .iter()
        .find(|tag| upper.contains(*tag))
        .map(|tag| tag.to_string())
}

/// Scan `models_dir` for `*.gguf` files. Ollama's `list_models()`
/// equivalent, but a directory listing instead of an `/api/tags` call -
/// there is no `client_for()` analog because there is no server to address.
/// An absent `models_dir` is treated as "no models yet" (empty list), not
/// an error - matches the directory not existing until the first download.
pub fn list_local_models(models_dir: &Path) -> Result<Vec<LocalGgufModel>> {
    if !models_dir.exists() {
        return Ok(Vec::new());
    }

    let mut models = Vec::new();
    for entry in std::fs::read_dir(models_dir)
        .with_context(|| format!("Failed to read models directory: {}", models_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("gguf") {
            continue;
        }

        let fs_metadata = entry.metadata()?;
        let modified_at = fs_metadata
            .modified()
            .ok()
            .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
            .unwrap_or_default();

        // Header-parsed metadata takes priority; the filename-convention
        // guess is only consulted as a last resort, when the header itself
        // couldn't be read (see `gguf_metadata::read_gguf_metadata`'s doc
        // comment for exactly when that happens).
        let header = super::gguf_metadata::read_gguf_metadata(&path);
        let quantization_hint = header
            .as_ref()
            .and_then(|h| h.quantization.clone())
            .or_else(|| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .and_then(quantization_hint_from_filename)
            });

        models.push(LocalGgufModel {
            path,
            size_bytes: fs_metadata.len(),
            modified_at,
            quantization_hint,
            architecture: header.as_ref().and_then(|h| h.architecture.clone()),
            parameter_count: header.as_ref().and_then(|h| h.parameter_count),
            context_length: header.as_ref().and_then(|h| h.context_length),
            has_chat_template: header.as_ref().is_some_and(|h| h.has_chat_template),
        });
    }

    models.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(models)
}

/// Parse an `hf:org/repo/file.gguf` shorthand into its parts. Pure
/// string-parsing, no I/O - `None` if `source` isn't in that shorthand
/// form (a direct URL, for instance).
fn parse_hf_shorthand(source: &str) -> Option<(&str, &str, &str)> {
    let rest = source.strip_prefix("hf:")?;
    let mut parts = rest.splitn(3, '/');
    let org = parts.next()?;
    let repo = parts.next()?;
    let file = parts.next()?;
    if org.is_empty() || repo.is_empty() || file.is_empty() {
        return None;
    }
    Some((org, repo, file))
}

/// Resolve `source` (a direct URL, or an `hf:org/repo/file.gguf`
/// shorthand) into a downloadable URL and, where resolvable, the file's
/// expected SHA-256 checksum.
///
/// For the `hf:` shorthand, this expands to
/// `https://huggingface.co/org/repo/resolve/main/file` and queries
/// Hugging Face's model API (`https://huggingface.co/api/models/org/repo`)
/// for the file's published LFS SHA-256 - a plain unauthenticated `GET`, no
/// HuggingFace client dependency needed. A direct URL has no metadata
/// endpoint to query at all, so it always resolves with `None` - callers
/// must treat that as "no integrity hash available", not silently skip the
/// gap (`llama-cpp-2-integration-plan.md` §4.11 point 2).
pub async fn resolve_download_source(source: &str) -> Result<(String, Option<String>)> {
    let Some((org, repo, file)) = parse_hf_shorthand(source) else {
        return Ok((source.to_string(), None));
    };

    let download_url = format!("https://huggingface.co/{org}/{repo}/resolve/main/{file}");
    let expected_sha256 = fetch_hf_lfs_sha256(org, repo, file).await;
    Ok((download_url, expected_sha256))
}

/// Best-effort lookup of a file's published SHA-256 from Hugging Face's
/// model metadata API. Returns `None` on any failure (network error,
/// non-LFS file, unexpected response shape) rather than propagating an
/// error - a missing checksum degrades to an explicit warning at the call
/// site, it does not block the download.
async fn fetch_hf_lfs_sha256(org: &str, repo: &str, file: &str) -> Option<String> {
    let api_url = format!("https://huggingface.co/api/models/{org}/{repo}");
    let response = reqwest::get(&api_url).await.ok()?;
    let body: serde_json::Value = response.json().await.ok()?;
    body.get("siblings")?
        .as_array()?
        .iter()
        .find(|s| s.get("rfilename").and_then(|f| f.as_str()) == Some(file))?
        .get("lfs")?
        .get("sha256")?
        .as_str()
        .map(str::to_string)
}

/// Download `url` into `models_dir`, streaming progress through
/// `progress_tx`. Ollama's `pull_model()` equivalent; same
/// `UnboundedSender<Progress>` callback shape, reusing the crate's own
/// `reqwest` (no isolated client needed - unlike `ollama-rs`, this module
/// has no conflicting `reqwest` major-version pin).
///
/// If `expected_sha256` is `Some`, the downloaded bytes are hashed
/// incrementally (never buffering the whole file in memory) and checked
/// before the file is left in `models_dir`; a mismatch deletes the partial
/// file and returns an error naming both hashes, rather than silently
/// keeping a corrupted/tampered file.
pub async fn download_model(
    url: &str,
    models_dir: &Path,
    expected_sha256: Option<&str>,
    progress_tx: UnboundedSender<DownloadProgress>,
) -> Result<PathBuf> {
    tokio::fs::create_dir_all(models_dir)
        .await
        .with_context(|| {
            format!(
                "Failed to create models directory: {}",
                models_dir.display()
            )
        })?;

    let filename = url
        .rsplit('/')
        .next()
        .filter(|s| !s.is_empty())
        .context("Could not determine a filename from the download URL")?;
    let final_path = models_dir.join(filename);
    let partial_path = models_dir.join(format!("{filename}.part"));

    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to start download from {url}"))?
        .error_for_status()
        .with_context(|| format!("Download failed for {url}"))?;
    let total_bytes = response.content_length();

    let mut file = tokio::fs::File::create(&partial_path)
        .await
        .with_context(|| format!("Failed to create {}", partial_path.display()))?;
    let mut hasher = Sha256::new();
    let mut bytes_downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Error while downloading")?;
        hasher.update(&chunk);
        file.write_all(&chunk)
            .await
            .with_context(|| format!("Failed to write to {}", partial_path.display()))?;
        bytes_downloaded += chunk.len() as u64;
        // Ignore send errors: the receiver (CLI printer / TUI dialog) may
        // have been dropped if the user cancelled or the view closed.
        let _ = progress_tx.send(DownloadProgress {
            bytes_downloaded,
            total_bytes,
        });
    }
    file.flush()
        .await
        .context("Failed to flush downloaded file")?;
    drop(file);

    if let Some(expected) = expected_sha256 {
        let actual = to_hex(&hasher.finalize());
        if !actual.eq_ignore_ascii_case(expected) {
            let _ = tokio::fs::remove_file(&partial_path).await;
            anyhow::bail!(
                "Checksum mismatch for {filename}: expected {expected}, got {actual}. \
                 The partial download has been deleted."
            );
        }
    }

    tokio::fs::rename(&partial_path, &final_path)
        .await
        .with_context(|| format!("Failed to finalize {}", final_path.display()))?;

    Ok(final_path)
}

/// Lowercase hex encoding, avoiding a dependency on the `hex` crate for
/// this one call site.
fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Delete a local `.gguf` file. Ollama's `delete_model()` equivalent.
pub fn delete_model(path: &Path) -> Result<()> {
    std::fs::remove_file(path).with_context(|| format!("Failed to delete {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantization_hint_recognizes_common_tags() {
        assert_eq!(
            quantization_hint_from_filename("Ornith-1.0-9B-Q4_K_M.gguf"),
            Some("Q4_K_M".to_string())
        );
        assert_eq!(
            quantization_hint_from_filename("llama-2-7b.Q8_0.gguf"),
            Some("Q8_0".to_string())
        );
        assert_eq!(
            quantization_hint_from_filename("model-f16.gguf"),
            Some("F16".to_string())
        );
    }

    #[test]
    fn quantization_hint_none_for_unrecognized_filename() {
        assert_eq!(
            quantization_hint_from_filename("my-custom-model.gguf"),
            None
        );
    }

    #[test]
    fn quantization_hint_is_case_insensitive() {
        assert_eq!(
            quantization_hint_from_filename("model-q4_k_m.GGUF"),
            Some("Q4_K_M".to_string())
        );
    }

    #[test]
    fn parse_hf_shorthand_extracts_org_repo_file() {
        assert_eq!(
            parse_hf_shorthand("hf:TheBloke/Llama-2-7B-GGUF/llama-2-7b.Q4_K_M.gguf"),
            Some(("TheBloke", "Llama-2-7B-GGUF", "llama-2-7b.Q4_K_M.gguf"))
        );
    }

    #[test]
    fn parse_hf_shorthand_none_for_direct_url() {
        assert_eq!(parse_hf_shorthand("https://example.com/model.gguf"), None);
    }

    #[test]
    fn parse_hf_shorthand_none_for_malformed_shorthand() {
        for bad in [
            "hf:",
            "hf:org",
            "hf:org/",
            "hf:org/repo",
            "hf:/repo/file",
            "hf:org//file",
        ] {
            assert_eq!(parse_hf_shorthand(bad), None, "should reject: {bad}");
        }
    }

    #[test]
    fn list_local_models_on_nonexistent_dir_returns_empty_not_error() {
        let dir = std::path::Path::new("/definitely/does/not/exist/crustly-test");
        let models = list_local_models(dir).expect("must not error on a missing dir");
        assert!(models.is_empty());
    }

    #[test]
    fn list_local_models_only_lists_gguf_files() {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::write(tmp.path().join("model-Q4_K_M.gguf"), b"fake gguf bytes")
            .expect("write gguf");
        std::fs::write(tmp.path().join("README.md"), b"not a model").expect("write readme");

        let models = list_local_models(tmp.path()).expect("list");
        assert_eq!(models.len(), 1);
        assert_eq!(models[0].quantization_hint, Some("Q4_K_M".to_string()));
        assert_eq!(models[0].size_bytes, "fake gguf bytes".len() as u64);
    }

    #[tokio::test]
    async fn resolve_download_source_passes_through_a_direct_url_unchanged() {
        let (url, sha) = resolve_download_source("https://example.com/model.gguf")
            .await
            .expect("resolve");
        assert_eq!(url, "https://example.com/model.gguf");
        assert_eq!(sha, None, "a direct URL has no known checksum endpoint");
    }

    #[tokio::test]
    async fn download_model_writes_the_file_and_reports_progress() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let body = b"fake gguf content for a download test".to_vec();
        let server = mock_http_server(body.clone()).await;

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let path = download_model(&format!("{server}/model.gguf"), tmp.path(), None, tx)
            .await
            .expect("download should succeed");

        assert_eq!(std::fs::read(&path).expect("read downloaded file"), body);
        assert!(!path.to_string_lossy().ends_with(".part"));

        let mut saw_progress = false;
        while let Ok(p) = rx.try_recv() {
            assert_eq!(p.bytes_downloaded as usize, body.len());
            saw_progress = true;
        }
        assert!(saw_progress, "expected at least one progress update");
    }

    #[tokio::test]
    async fn download_model_rejects_a_checksum_mismatch_and_cleans_up() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let body = b"fake gguf content".to_vec();
        let server = mock_http_server(body).await;

        let (tx, _rx) = tokio::sync::mpsc::unbounded_channel();
        let err = download_model(
            &format!("{server}/model.gguf"),
            tmp.path(),
            Some("0000000000000000000000000000000000000000000000000000000000000000"),
            tx,
        )
        .await
        .expect_err("checksum mismatch must error");

        assert!(err.to_string().contains("Checksum mismatch"));
        assert!(
            !tmp.path().join("model.gguf.part").exists(),
            "the partial file must be cleaned up on mismatch"
        );
        assert!(!tmp.path().join("model.gguf").exists());
    }

    /// Minimal single-request HTTP mock server (mirrors
    /// `ollama_models.rs`'s `mock_server()` pattern - no external mocking
    /// crate dependency), serving `body` as a 200 response with a correct
    /// Content-Length to any request, then shutting down.
    async fn mock_http_server(body: Vec<u8>) -> String {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock server");
        let addr = listener.local_addr().expect("mock server addr");

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.expect("accept request");

            let mut request = Vec::new();
            let mut buf = [0u8; 1024];
            loop {
                let n = socket.read(&mut buf).await.expect("read request");
                if n == 0 {
                    break;
                }
                request.extend_from_slice(&buf[..n]);
                if request.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            socket
                .write_all(response.as_bytes())
                .await
                .expect("write response headers");
            socket.write_all(&body).await.expect("write response body");
            socket.shutdown().await.ok();
        });

        format!("http://{addr}")
    }
}

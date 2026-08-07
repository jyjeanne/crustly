//! `llama.cpp` model management (list/download/delete local `.gguf` files).
//!
//! Unlike `ollama_models.rs`, there is no server to address - this is local
//! filesystem scanning plus a plain HTTP download, named and shaped to
//! match `ollama_models.rs` field-for-field (`llama-cpp-2-integration-plan.md`
//! §8) rather than inventing new vocabulary for the same kind of operation.

use anyhow::{Context, Result};
use futures::StreamExt as _;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt as _;
use tokio::sync::mpsc::UnboundedSender;

/// Context length used for `list_local_models`'s memory estimate when a
/// model's own header doesn't set one (or sets an implausibly large one) -
/// matches `default_llama_cpp_n_ctx()` (`src/config/mod.rs`), so the
/// estimate reflects what a freshly-configured provider would actually
/// allocate, not the model's full trained context (which for some modern
/// models is far larger than anyone would default to in practice).
const ESTIMATE_DEFAULT_CONTEXT_LENGTH: u64 = 8_192;

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
    /// A name to display instead of `path`'s filename, when the filename
    /// itself isn't meaningful - e.g. an Ollama-sourced entry's real path
    /// is a content-addressed blob (`sha256-<hex>`), so this carries the
    /// manifest's own `name:tag`. Comma-joined when more than one manifest
    /// tag resolves to the identical blob. `None` for ordinary
    /// filesystem-scanned entries, where the filename already is the
    /// name; callers fall back to `path`'s filename in that case, exactly
    /// as they did before this field existed.
    pub display_name: Option<String>,
    /// Estimated resident memory (weights + KV cache when the header has
    /// enough geometry) at `ESTIMATE_DEFAULT_CONTEXT_LENGTH` tokens, or the
    /// header's own native context length if smaller. `None` if
    /// `parameter_count`/`quantization_hint` weren't determined - see
    /// `gguf_metadata::estimate_memory_bytes`'s doc comment for the
    /// "order-of-magnitude, not a guarantee" framing.
    pub estimated_memory_bytes: Option<u64>,
    /// Whether `estimated_memory_bytes` includes the KV-cache term.
    /// `false` when the header lacked the attention geometry to compute
    /// it, in which case the estimate covers weights only - meaningless
    /// when `estimated_memory_bytes` is `None`.
    pub estimated_memory_includes_kv_cache: bool,
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

        let estimate = header.as_ref().and_then(|h| {
            let ctx = h
                .context_length
                .map(|native| native.min(ESTIMATE_DEFAULT_CONTEXT_LENGTH))
                .unwrap_or(ESTIMATE_DEFAULT_CONTEXT_LENGTH);
            super::gguf_metadata::estimate_memory_bytes(h, ctx)
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
            display_name: None,
            estimated_memory_bytes: estimate.map(|e| e.total_bytes),
            estimated_memory_includes_kv_cache: estimate.is_some_and(|e| e.includes_kv_cache),
        });
    }

    models.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(models)
}

/// Scan every configured source and merge the results: `models_dir`,
/// `extra_model_paths`, and (when `ollama_models_dir` is `Some` -
/// `Config::providers::llama_cpp_ollama_models_dir()` already gates this on
/// the `scan_ollama_models` opt-in) Ollama's manifest tree. This is the
/// entry point the CLI and TUI should use instead of `list_local_models`
/// directly; `list_local_models` itself is unchanged (still a single
/// directory, still what this function calls per source).
///
/// A failure scanning one `extra_model_paths` entry fails the whole call
/// (same "surface real errors" posture `list_local_models` already has for
/// its one directory) - but the Ollama scan is best-effort and never fails
/// the call, since a missing/unreadable manifest tree just means "nothing
/// found there," not a problem with the directories the user explicitly
/// configured.
pub fn list_all_local_models(
    models_dir: &Path,
    extra_model_paths: &[PathBuf],
    ollama_models_dir: Option<&Path>,
) -> Result<Vec<LocalGgufModel>> {
    let mut models = list_local_models(models_dir)?;
    for extra in extra_model_paths {
        models.extend(list_local_models(extra)?);
    }
    if let Some(ollama_dir) = ollama_models_dir {
        models.extend(list_ollama_models(ollama_dir));
    }

    let mut models = deduplicate_and_merge(models);
    models.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(models)
}

/// One layer entry in an Ollama manifest's `layers` array.
#[derive(Debug, Deserialize)]
struct OllamaManifestLayer {
    #[serde(rename = "mediaType")]
    media_type: String,
    digest: String,
}

/// The subset of an Ollama manifest's JSON this module reads. Verified
/// against Ollama's own source (`server/manifest.go`/`server/images.go`,
/// `types/model/name.go`) rather than assumed - see
/// `ccguf-managment-imrpoment-plan.md` Phase M3.
#[derive(Debug, Deserialize)]
struct OllamaManifest {
    layers: Vec<OllamaManifestLayer>,
}

/// The GGUF model-weights layer's media type - distinct from
/// `application/vnd.ollama.image.projector` (mmproj, out of scope until
/// Phase M5), `.../adapter`, `.../template`, `.../license`, etc.
const OLLAMA_MODEL_LAYER_MEDIA_TYPE: &str = "application/vnd.ollama.image.model";

/// Discover models Ollama has already pulled by reading its manifest tree
/// at `<ollama_models_dir>/manifests/{host}/{namespace}/{model}/{tag}` -
/// always exactly 4 path components deep (verified against Ollama's
/// `Filepath()` layout), so a fixed 4-level `read_dir` nest finds every
/// manifest without a general recursive walker. Each manifest resolves to
/// its model-weights blob (`<ollama_models_dir>/blobs/sha256-<hex>`, colon
/// replaced with a dash - verified against Ollama's own blob-naming code)
/// and a human-readable `display_name` reconstructed from the manifest's
/// own path, not guessed from the blob's content-addressed filename.
///
/// Best-effort: a missing `manifests/` directory returns an empty list (not
/// an error - this is likely just "Ollama isn't installed" or "opted in
/// without Ollama ever having pulled anything"); an individual unreadable
/// or malformed manifest, or one whose blob is missing (pruned since the
/// manifest was written), is skipped rather than failing the whole scan.
fn list_ollama_models(ollama_models_dir: &Path) -> Vec<LocalGgufModel> {
    let manifests_root = ollama_models_dir.join("manifests");
    let blobs_root = ollama_models_dir.join("blobs");

    walk_ollama_manifest_files(&manifests_root)
        .into_iter()
        .filter_map(|manifest_path| {
            parse_ollama_manifest(&manifest_path, &manifests_root, &blobs_root)
        })
        .collect()
}

/// Fixed 4-level directory walk (`host/namespace/model/tag`) - deliberately
/// not a general recursive walker: Ollama's manifest layout is always
/// exactly this deep, so this is both simpler and immune to the
/// symlink-loop/unbounded-depth concerns a generic walker would need to
/// guard against.
fn walk_ollama_manifest_files(manifests_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(hosts) = std::fs::read_dir(manifests_root) else {
        return files;
    };
    for host in hosts.flatten() {
        let Ok(namespaces) = std::fs::read_dir(host.path()) else {
            continue;
        };
        for namespace in namespaces.flatten() {
            let Ok(models) = std::fs::read_dir(namespace.path()) else {
                continue;
            };
            for model in models.flatten() {
                let Ok(tags) = std::fs::read_dir(model.path()) else {
                    continue;
                };
                for tag in tags.flatten() {
                    let path = tag.path();
                    if path.is_file() {
                        files.push(path);
                    }
                }
            }
        }
    }
    files
}

/// Parse one manifest file into a `LocalGgufModel`. `None` on any problem
/// (unreadable, malformed JSON, no model layer, unresolvable digest,
/// missing blob, or a path that isn't the expected 4 components deep under
/// `manifests_root`) - the caller (`list_ollama_models`) treats that as
/// "skip this one," not a hard failure.
fn parse_ollama_manifest(
    manifest_path: &Path,
    manifests_root: &Path,
    blobs_root: &Path,
) -> Option<LocalGgufModel> {
    let bytes = std::fs::read(manifest_path).ok()?;
    let manifest: OllamaManifest = serde_json::from_slice(&bytes).ok()?;
    let model_layer = manifest
        .layers
        .iter()
        .find(|l| l.media_type == OLLAMA_MODEL_LAYER_MEDIA_TYPE)?;

    let hex_digest = model_layer.digest.strip_prefix("sha256:")?;
    if hex_digest.is_empty() || !hex_digest.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let blob_path = blobs_root.join(format!("sha256-{hex_digest}"));
    let fs_metadata = std::fs::metadata(&blob_path).ok()?; // also confirms the blob exists

    let rel = manifest_path.strip_prefix(manifests_root).ok()?;
    let parts: Vec<&str> = rel
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    let [_host, namespace, model, tag] = parts[..] else {
        return None;
    };
    let display_name = if namespace == "library" {
        format!("{model}:{tag}")
    } else {
        format!("{namespace}/{model}:{tag}")
    };

    let modified_at = fs_metadata
        .modified()
        .ok()
        .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
        .unwrap_or_default();

    // An Ollama blob's own bytes are a plain GGUF file - the same header
    // parser applies directly, no Ollama-specific decoding needed for the
    // model's own contents.
    let header = super::gguf_metadata::read_gguf_metadata(&blob_path);
    let quantization_hint = header.as_ref().and_then(|h| h.quantization.clone());
    let estimate = header.as_ref().and_then(|h| {
        let ctx = h
            .context_length
            .map(|native| native.min(ESTIMATE_DEFAULT_CONTEXT_LENGTH))
            .unwrap_or(ESTIMATE_DEFAULT_CONTEXT_LENGTH);
        super::gguf_metadata::estimate_memory_bytes(h, ctx)
    });

    Some(LocalGgufModel {
        path: blob_path,
        size_bytes: fs_metadata.len(),
        modified_at,
        quantization_hint,
        architecture: header.as_ref().and_then(|h| h.architecture.clone()),
        parameter_count: header.as_ref().and_then(|h| h.parameter_count),
        context_length: header.as_ref().and_then(|h| h.context_length),
        has_chat_template: header.as_ref().is_some_and(|h| h.has_chat_template),
        display_name: Some(display_name),
        estimated_memory_bytes: estimate.map(|e| e.total_bytes),
        estimated_memory_includes_kv_cache: estimate.is_some_and(|e| e.includes_kv_cache),
    })
}

/// Detects the `<base>-00001-of-00005.gguf` split-file naming convention
/// (both numbers zero-padded to the same width, per
/// `ccguf-managment-imrpoment-plan.md` Phase M4). Pure string parsing, no
/// regex dependency needed. Returns `(base, part_index, total_parts)` -
/// `part_index` is 1-based, matching the filename's own convention. `None`
/// if `filename` doesn't match (including a width mismatch between the two
/// numbers, which the convention requires).
fn parse_split_gguf_filename(filename: &str) -> Option<(&str, u32, u32)> {
    let stem = filename.strip_suffix(".gguf")?;
    let of_pos = stem.rfind("-of-")?;
    let (before_of, total_str) = (&stem[..of_pos], &stem[of_pos + 4..]);
    if total_str.is_empty() || !total_str.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let dash_pos = before_of.rfind('-')?;
    let (base, part_str) = (&before_of[..dash_pos], &before_of[dash_pos + 1..]);
    if part_str.len() != total_str.len() || !part_str.bytes().all(|b| b.is_ascii_digit()) {
        return None; // widths must match - that's the whole convention
    }

    let part: u32 = part_str.parse().ok()?;
    let total: u32 = total_str.parse().ok()?;
    if part == 0 || part > total {
        return None;
    }
    Some((base, part, total))
}

/// Post-process a merged listing from `list_all_local_models`'s sources:
///
/// 1. **Canonical-path dedup** - the same underlying file reached two ways
///    (a symlink in one scanned directory pointing at a file already found
///    via another, or two Ollama manifest tags pointing at the identical
///    blob) collapses to one entry. `display_name`s combine
///    (comma-joined) rather than one being silently dropped; the entry
///    with the richer header (`architecture.is_some()`, first-seen as a
///    tiebreak) "wins" for the other fields. Sizes are **not** summed here
///    - it's the same file counted once, not two files.
/// 2. **Split-GGUF grouping** - `-00001-of-00005.gguf`-convention parts
///    collapse into one logical entry: canonical `path` is the
///    lowest-found part index (GGUF's split convention puts the real
///    header/metadata in part 1; later shards carry only bookkeeping
///    keys), `size_bytes` is **summed** (these are genuinely separate
///    files), `display_name` is the base name, and metadata comes from
///    whichever found part has it (same "richest wins" rule) - a partial
///    download (not all parts present yet) is handled honestly rather
///    than requiring completeness.
///
/// `O(n²)` over the input - the expected entry count (tens, not thousands
/// of models on one machine) doesn't justify a hash-based grouping
/// implementation's added complexity here.
fn deduplicate_and_merge(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel> {
    let by_canonical_path = merge_by_canonical_path(models);
    merge_split_gguf_groups(by_canonical_path)
}

/// True if `richer` has more identifying header data than `current` -
/// the tiebreak `deduplicate_and_merge`'s two passes both use to decide
/// which of two entries for "the same logical model" supplies the
/// metadata fields on the merged result.
fn is_richer(candidate: &LocalGgufModel, current: &LocalGgufModel) -> bool {
    candidate.architecture.is_some() && current.architecture.is_none()
}

fn merge_by_canonical_path(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel> {
    let mut by_path: HashMap<PathBuf, LocalGgufModel> = HashMap::new();
    let mut order: Vec<PathBuf> = Vec::new();

    for model in models {
        let key = std::fs::canonicalize(&model.path).unwrap_or_else(|_| model.path.clone());
        match by_path.get_mut(&key) {
            None => {
                order.push(key.clone());
                by_path.insert(key, model);
            }
            Some(existing) => {
                let merged_display_name = match (&existing.display_name, &model.display_name) {
                    (Some(a), Some(b)) if a != b => Some(format!("{a}, {b}")),
                    (Some(a), _) => Some(a.clone()),
                    (None, Some(b)) => Some(b.clone()),
                    (None, None) => None,
                };
                if is_richer(&model, existing) {
                    let display_name = merged_display_name;
                    *existing = LocalGgufModel {
                        display_name,
                        ..model
                    };
                } else {
                    existing.display_name = merged_display_name;
                }
            }
        }
    }

    order
        .into_iter()
        .filter_map(|key| by_path.remove(&key))
        .collect()
}

fn merge_split_gguf_groups(models: Vec<LocalGgufModel>) -> Vec<LocalGgufModel> {
    let mut groups: HashMap<(PathBuf, String, u32), Vec<LocalGgufModel>> = HashMap::new();
    let mut order: Vec<(PathBuf, String, u32)> = Vec::new();
    let mut passthrough: Vec<LocalGgufModel> = Vec::new();

    for model in models {
        let filename = model
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .map(str::to_string);
        let parent = model
            .path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        let split = filename
            .as_deref()
            .and_then(parse_split_gguf_filename)
            .map(|(base, part, total)| (base.to_string(), part, total));

        match split {
            None => passthrough.push(model),
            Some((base, _part, total)) => {
                let key = (parent, base, total);
                if !groups.contains_key(&key) {
                    order.push(key.clone());
                }
                groups.entry(key).or_default().push(model);
            }
        }
    }

    let mut result = passthrough;
    for key in order {
        let Some(mut parts) = groups.remove(&key) else {
            continue;
        };
        parts.sort_by_key(|m| {
            m.path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(parse_split_gguf_filename)
                .map(|(_, part, _)| part)
                .unwrap_or(u32::MAX)
        });

        let (_parent, base, _total) = key;
        let summed_size = parts.iter().map(|p| p.size_bytes).sum();
        let template_index = parts
            .iter()
            .enumerate()
            .max_by_key(|(_, m)| m.architecture.is_some())
            .map(|(i, _)| i)
            .unwrap_or(0);
        let canonical_path = parts[0].path.clone();
        let modified_at = parts[0].modified_at.clone();
        let mut merged = parts[template_index].clone();
        merged.path = canonical_path;
        merged.modified_at = modified_at;
        merged.size_bytes = summed_size;
        merged.display_name = Some(format!("{base}.gguf"));

        result.push(merged);
    }

    result
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

    // --- M3: multi-source discovery ---

    #[test]
    fn list_all_local_models_merges_models_dir_and_extra_paths() {
        let primary = tempfile::tempdir().expect("tempdir");
        let extra = tempfile::tempdir().expect("tempdir");
        std::fs::write(primary.path().join("a.gguf"), b"gguf a").expect("write a");
        std::fs::write(extra.path().join("b.gguf"), b"gguf b").expect("write b");

        let models = list_all_local_models(primary.path(), &[extra.path().to_path_buf()], None)
            .expect("list");

        let names: Vec<_> = models
            .iter()
            .map(|m| m.path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(models.len(), 2);
        assert!(names.contains(&"a.gguf".to_string()));
        assert!(names.contains(&"b.gguf".to_string()));
    }

    #[test]
    fn list_all_local_models_with_no_ollama_dir_does_not_scan_ollama() {
        let primary = tempfile::tempdir().expect("tempdir");
        std::fs::write(primary.path().join("a.gguf"), b"gguf a").expect("write a");

        let models = list_all_local_models(primary.path(), &[], None).expect("list");
        assert_eq!(models.len(), 1);
    }

    // --- M3: Ollama manifest discovery ---

    /// Builds a fake `<root>/manifests/<host>/<namespace>/<model>/<tag>`
    /// manifest file naming `hex_digest` as its model layer, plus the
    /// matching `<root>/blobs/sha256-<hex_digest>` blob file with
    /// `blob_body` as its content.
    fn write_fake_ollama_model(
        root: &Path,
        host: &str,
        namespace: &str,
        model: &str,
        tag: &str,
        hex_digest: &str,
        blob_body: &[u8],
    ) {
        let manifest_dir = root
            .join("manifests")
            .join(host)
            .join(namespace)
            .join(model);
        std::fs::create_dir_all(&manifest_dir).expect("create manifest dir");
        let manifest_json = format!(
            r#"{{"schemaVersion":2,"layers":[
                {{"mediaType":"application/vnd.ollama.image.model","digest":"sha256:{hex_digest}","size":{size}}},
                {{"mediaType":"application/vnd.ollama.image.template","digest":"sha256:{hex_digest}","size":1}}
            ]}}"#,
            size = blob_body.len()
        );
        std::fs::write(manifest_dir.join(tag), manifest_json).expect("write manifest");

        let blobs_dir = root.join("blobs");
        std::fs::create_dir_all(&blobs_dir).expect("create blobs dir");
        std::fs::write(blobs_dir.join(format!("sha256-{hex_digest}")), blob_body)
            .expect("write blob");
    }

    const FAKE_DIGEST_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn list_ollama_models_resolves_library_namespace_display_name() {
        let root = tempfile::tempdir().expect("tempdir");
        write_fake_ollama_model(
            root.path(),
            "registry.ollama.ai",
            "library",
            "qwen2.5-coder",
            "7b",
            FAKE_DIGEST_A,
            b"fake gguf blob content",
        );

        let models = list_ollama_models(root.path());
        assert_eq!(models.len(), 1);
        assert_eq!(
            models[0].display_name.as_deref(),
            Some("qwen2.5-coder:7b"),
            "the default 'library' namespace is omitted, matching Ollama's own display convention"
        );
        assert_eq!(models[0].size_bytes, "fake gguf blob content".len() as u64);
    }

    #[test]
    fn list_ollama_models_keeps_non_library_namespace_in_display_name() {
        let root = tempfile::tempdir().expect("tempdir");
        write_fake_ollama_model(
            root.path(),
            "registry.ollama.ai",
            "myorg",
            "custom-model",
            "v1",
            FAKE_DIGEST_A,
            b"fake gguf blob content",
        );

        let models = list_ollama_models(root.path());
        assert_eq!(models.len(), 1);
        assert_eq!(
            models[0].display_name.as_deref(),
            Some("myorg/custom-model:v1")
        );
    }

    #[test]
    fn list_ollama_models_on_missing_manifests_dir_returns_empty_not_error() {
        let root = tempfile::tempdir().expect("tempdir");
        assert!(list_ollama_models(root.path()).is_empty());
    }

    #[test]
    fn list_ollama_models_skips_a_manifest_whose_blob_is_missing() {
        let root = tempfile::tempdir().expect("tempdir");
        let manifest_dir = root
            .path()
            .join("manifests")
            .join("registry.ollama.ai")
            .join("library")
            .join("pruned-model");
        std::fs::create_dir_all(&manifest_dir).expect("create manifest dir");
        std::fs::write(
            manifest_dir.join("latest"),
            format!(
                r#"{{"layers":[{{"mediaType":"application/vnd.ollama.image.model","digest":"sha256:{FAKE_DIGEST_A}","size":1}}]}}"#
            ),
        )
        .expect("write manifest");
        // Deliberately no matching blob file written.

        assert!(list_ollama_models(root.path()).is_empty());
    }

    #[test]
    fn list_all_local_models_merges_two_manifest_tags_sharing_one_blob() {
        let root = tempfile::tempdir().expect("tempdir");
        write_fake_ollama_model(
            root.path(),
            "registry.ollama.ai",
            "library",
            "qwen2.5-coder",
            "7b",
            FAKE_DIGEST_A,
            b"shared blob content",
        );
        write_fake_ollama_model(
            root.path(),
            "registry.ollama.ai",
            "library",
            "qwen2.5-coder",
            "latest",
            FAKE_DIGEST_A, // same digest as above - the identical blob, two tags
            b"shared blob content",
        );

        let empty_dir = tempfile::tempdir().expect("tempdir");
        let models = list_all_local_models(empty_dir.path(), &[], Some(root.path())).expect("list");

        assert_eq!(
            models.len(),
            1,
            "two tags pointing at the identical blob must collapse to one entry"
        );
        let name = models[0].display_name.as_deref().unwrap_or_default();
        assert!(name.contains("qwen2.5-coder:7b"));
        assert!(name.contains("qwen2.5-coder:latest"));
        assert_eq!(
            models[0].size_bytes,
            "shared blob content".len() as u64,
            "size must not be double-counted for the same underlying blob"
        );
    }

    // --- M4: symlink dedup ---

    #[cfg(unix)]
    #[test]
    fn list_all_local_models_collapses_a_symlink_to_an_already_found_file() {
        let primary = tempfile::tempdir().expect("tempdir");
        let extra = tempfile::tempdir().expect("tempdir");
        let real_path = primary.path().join("real-model.gguf");
        std::fs::write(&real_path, b"gguf content").expect("write real file");
        std::os::unix::fs::symlink(&real_path, extra.path().join("real-model.gguf"))
            .expect("create symlink");

        let models = list_all_local_models(primary.path(), &[extra.path().to_path_buf()], None)
            .expect("list");

        assert_eq!(
            models.len(),
            1,
            "a symlink to an already-discovered file must not double-count it"
        );
        assert_eq!(models[0].size_bytes, "gguf content".len() as u64);
    }

    // --- M4: split-GGUF filename parsing and grouping ---

    #[test]
    fn parse_split_gguf_filename_matches_the_convention() {
        assert_eq!(
            parse_split_gguf_filename("model-00001-of-00005.gguf"),
            Some(("model", 1, 5))
        );
        assert_eq!(
            parse_split_gguf_filename("Qwen-235B-00012-of-00030.gguf"),
            Some(("Qwen-235B", 12, 30))
        );
    }

    #[test]
    fn parse_split_gguf_filename_rejects_mismatched_widths() {
        // "1" (1 digit) vs "00005" (5 digits) - the convention requires
        // equal-width zero-padding on both sides.
        assert_eq!(parse_split_gguf_filename("model-1-of-00005.gguf"), None);
    }

    #[test]
    fn parse_split_gguf_filename_rejects_non_split_names_and_out_of_range_parts() {
        for name in [
            "model.gguf",
            "model-Q4_K_M.gguf",
            "model-of-00005.gguf",       // no part number before "-of-"
            "model-00000-of-00005.gguf", // part 0 is out of range (1-based)
            "model-00006-of-00005.gguf", // part exceeds total
        ] {
            assert_eq!(
                parse_split_gguf_filename(name),
                None,
                "should reject: {name}"
            );
        }
    }

    #[test]
    fn list_all_local_models_collapses_a_split_gguf_group() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("big-model-00001-of-00003.gguf"),
            b"part one",
        )
        .expect("write part 1");
        std::fs::write(
            dir.path().join("big-model-00002-of-00003.gguf"),
            b"part two!",
        )
        .expect("write part 2");
        std::fs::write(
            dir.path().join("big-model-00003-of-00003.gguf"),
            b"part three!!",
        )
        .expect("write part 3");

        let models = list_all_local_models(dir.path(), &[], None).expect("list");

        assert_eq!(models.len(), 1, "three parts must collapse to one entry");
        assert_eq!(models[0].display_name.as_deref(), Some("big-model.gguf"));
        assert_eq!(
            models[0].size_bytes,
            b"part one".len() as u64 + b"part two!".len() as u64 + b"part three!!".len() as u64
        );
        assert!(
            models[0]
                .path
                .to_string_lossy()
                .ends_with("big-model-00001-of-00003.gguf"),
            "the canonical path should be the lowest-found part"
        );
    }

    #[test]
    fn list_all_local_models_handles_a_partial_split_group_honestly() {
        let dir = tempfile::tempdir().expect("tempdir");
        // Only part 2 of 3 is present - e.g. an interrupted multi-part
        // download.
        std::fs::write(
            dir.path().join("big-model-00002-of-00003.gguf"),
            b"part two!",
        )
        .expect("write part 2");

        let models = list_all_local_models(dir.path(), &[], None).expect("list");

        assert_eq!(models.len(), 1);
        assert_eq!(models[0].size_bytes, b"part two!".len() as u64);
        assert!(
            models[0]
                .path
                .to_string_lossy()
                .ends_with("big-model-00002-of-00003.gguf"),
            "with only one part found, it is its own canonical path"
        );
    }

    #[test]
    fn list_all_local_models_does_not_group_unrelated_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::write(dir.path().join("model-a.gguf"), b"a").expect("write a");
        std::fs::write(dir.path().join("model-b.gguf"), b"b").expect("write b");

        let models = list_all_local_models(dir.path(), &[], None).expect("list");
        assert_eq!(
            models.len(),
            2,
            "ordinary unrelated files must stay separate"
        );
    }
}

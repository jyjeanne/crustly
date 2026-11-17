//! Configuration types for providers in this crate. Crustly re-exports them from
//! `crustly::config`, so its config file keeps the same shape.

use serde::{Deserialize, Serialize};

/// In-process `llama.cpp` provider configuration (`feature = "llama-cpp"`).
///
/// Unlike `OllamaProviderConfig`, this binds to exactly one loaded `.gguf`
/// file per provider instance - there is no server-side model registry to
/// resolve a name against, so `model_path` (not a model name) is the
/// required field. See `llama-cpp-2-integration-plan.md` §5.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlamaCppProviderConfig {
    /// Provider enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Path to a local `.gguf` model file. Required - unlike Ollama, there
    /// is no name-based model registry to resolve against.
    pub model_path: std::path::PathBuf,

    /// Display name for the model (defaults to `model_path`'s file stem,
    /// e.g. "qwen2.5-coder-7b-instruct-q4_k_m", when unset).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

    /// Context window size, fixed for the life of the loaded context -
    /// unlike Ollama's per-request `num_ctx`, changing this requires a
    /// model reload (see `LlamaCppProvider`'s model-swap cost).
    #[serde(default = "default_llama_cpp_n_ctx")]
    pub n_ctx: u32,

    /// Number of model layers to offload to GPU. 0 = CPU only (default,
    /// always buildable). Ignored (logged once) if the binary was not built
    /// with a GPU feature (`llama-cpp-cuda`/`-metal`/`-vulkan`/...).
    #[serde(default)]
    pub n_gpu_layers: u32,

    /// CPU thread count for decode. Defaults to the number of physical
    /// cores if unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_threads: Option<u32>,

    /// Optional chat-template override (raw template string). When unset,
    /// the model's own embedded GGUF chat template is used if present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_template: Option<String>,

    /// Sampling defaults - a local model rarely behaves well on generic
    /// defaults, same rationale as `OllamaProviderConfig`'s equivalents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Auto-unload the model after this many idle seconds. Unset = never.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_unload_secs: Option<u64>,

    /// Directory scanned/used by the `crustly llama-cpp` model-management
    /// commands for listing and downloading `.gguf` files. Defaults to a
    /// platform cache dir (e.g. `~/.cache/crustly/models` on Linux) when
    /// unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models_dir: Option<std::path::PathBuf>,

    /// Extra directories the `llama-cpp` model-management commands also
    /// scan for `.gguf` files, beyond `models_dir`. Empty by default -
    /// see `ccguf-managment-imrpoment-plan.md` Phase M3.
    #[serde(default)]
    pub extra_model_paths: Vec<std::path::PathBuf>,

    /// Also discover models Ollama has already pulled, by reading its
    /// manifest tree (`$OLLAMA_MODELS` or `~/.ollama/models`) rather than
    /// guessing blob names - not a raw blob-store scan. `false` by
    /// default: this is an explicit opt-in, independent of whether this
    /// build was compiled with the `ollama` Cargo feature (that feature
    /// only governs talking to an Ollama *server*; whether an Ollama
    /// *daemon* left models on disk is unrelated) - see Phase M3's
    /// correction in `ccguf-managment-imrpoment-plan.md`.
    #[serde(default)]
    pub scan_ollama_models: bool,
}

impl Default for LlamaCppProviderConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            model_path: std::path::PathBuf::new(),
            display_name: None,
            n_ctx: default_llama_cpp_n_ctx(),
            n_gpu_layers: 0,
            n_threads: None,
            chat_template: None,
            temperature: None,
            top_p: None,
            top_k: None,
            repeat_penalty: None,
            seed: None,
            idle_unload_secs: None,
            models_dir: None,
            extra_model_paths: Vec::new(),
            scan_ollama_models: false,
        }
    }
}

fn default_llama_cpp_n_ctx() -> u32 {
    8_192
}

fn default_enabled() -> bool {
    true
}

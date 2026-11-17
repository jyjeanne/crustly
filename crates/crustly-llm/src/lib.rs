//! LLM provider layer of [Crustly](https://github.com/jyjeanne/crustly), shared with
//! other projects.
//!
//! One [`Provider`] trait over Anthropic, OpenAI and OpenAI-compatible servers
//! (LM Studio, Ollama's `/v1` endpoint, LocalAI), native Ollama, Gemini, Azure
//! OpenAI, Qwen and in-process llama.cpp, with token-by-token streaming and tool
//! calls.
//!
//! Cargo features:
//! - `ollama`: native Ollama API through `ollama-rs`
//! - `gguf-management`: local `.gguf` model files and hardware detection
//! - `llama-cpp`: in-process llama.cpp (builds C++, needs CMake), with GPU
//!   variants `llama-cpp-cuda`, `-metal`, `-vulkan`, `-rocm`, `-opencl`, `-mkl`

pub mod config;
pub mod error;
pub mod model_hints;
pub mod retry;
pub mod router;
pub mod tool_call_recovery;
#[allow(clippy::module_inception)]
mod r#trait;
pub mod types;

// Re-exports
pub use error::{ProviderError, Result};
pub use r#trait::{Provider, ProviderCapabilities, ProviderStream};
pub use types::*;

// Provider implementations
pub mod anthropic;
pub mod azure;
pub mod gemini;
#[cfg(feature = "gguf-management")]
pub mod gguf_metadata;
#[cfg(feature = "gguf-management")]
pub mod hardware_detect;
#[cfg(feature = "llama-cpp")]
pub mod llama_cpp;
#[cfg(all(feature = "llama-cpp", feature = "llguidance-grammar"))]
pub mod llama_cpp_grammar;
#[cfg(feature = "gguf-management")]
pub mod llama_cpp_models;
#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "ollama")]
pub mod ollama_models;
pub mod openai;
pub mod prompt_cache;
pub mod qwen;
#[cfg(test)]
mod test_support;

pub use anthropic::AnthropicProvider;
pub use azure::AzureOpenAIProvider;
pub use gemini::GeminiProvider;
#[cfg(feature = "llama-cpp")]
pub use llama_cpp::LlamaCppProvider;
#[cfg(feature = "ollama")]
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;
pub use qwen::{QwenProvider, ThinkingConfig, ToolCallParser};

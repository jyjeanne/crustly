//! LLM Provider Abstraction Layer
//!
//! Provides a unified interface for interacting with different LLM providers.

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
pub mod factory;
pub mod gemini;
#[cfg(feature = "gguf-management")]
pub mod gguf_metadata;
#[cfg(feature = "llama-cpp")]
pub mod llama_cpp;
#[cfg(feature = "llama-cpp-llguidance")]
pub mod llama_cpp_grammar;
#[cfg(feature = "gguf-management")]
pub mod llama_cpp_models;
#[cfg(feature = "ollama")]
pub mod ollama;
#[cfg(feature = "ollama")]
pub mod ollama_models;
pub mod openai;
pub mod qwen;

pub use anthropic::AnthropicProvider;
pub use azure::AzureOpenAIProvider;
pub use factory::create_provider;
#[cfg(feature = "ollama")]
pub use factory::ollama_provider_from_config;
pub use gemini::GeminiProvider;
#[cfg(feature = "llama-cpp")]
pub use llama_cpp::LlamaCppProvider;
#[cfg(feature = "ollama")]
pub use ollama::OllamaProvider;
pub use openai::OpenAIProvider;
pub use qwen::{QwenProvider, ThinkingConfig, ToolCallParser};

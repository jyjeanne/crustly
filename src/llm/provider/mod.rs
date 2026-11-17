//! LLM Provider Abstraction Layer
//!
//! The providers live in the `crustly-llm` crate (`crates/crustly-llm`), which
//! other projects use as well. This module re-exports it, so every
//! `crate::llm::provider::...` path keeps working, and adds the factory that
//! builds a provider from Crustly's [`Config`](crate::config::Config).

pub use crustly_llm::*;

pub mod factory;

#[cfg(feature = "ollama")]
pub use factory::ollama_provider_from_config;
pub use factory::{create_named_provider, create_provider};

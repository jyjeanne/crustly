//! Provider Factory
//!
//! Centralized provider creation logic to reduce code duplication.

use super::{
    anthropic::AnthropicProvider,
    error::ProviderError,
    gemini::GeminiProvider,
    openai::OpenAIProvider,
    qwen::{QwenProvider, ToolCallParser},
    Provider,
};
use crate::config::{Config, ProviderConfig, QwenProviderConfig};
use anyhow::{Context, Result};
use async_trait::async_trait;
use std::sync::Arc;

// ── Failover provider ──────────────────────────────────────────────────────────

/// A `Provider` wrapper that retries with secondary providers on transient errors.
///
/// On `RateLimitExceeded`, `Timeout`, or 5xx `ApiError`, it logs `[FAILOVER]`
/// and calls the next provider in the chain.
pub struct FailoverProvider {
    chain: Vec<Arc<dyn Provider>>,
}

impl FailoverProvider {
    /// Build a failover chain. At least two providers are required.
    pub fn new(chain: Vec<Arc<dyn Provider>>) -> Self {
        assert!(!chain.is_empty(), "failover chain must not be empty");
        Self { chain }
    }

    fn is_failover_error(err: &ProviderError) -> bool {
        matches!(
            err,
            ProviderError::RateLimitExceeded(_)
                | ProviderError::Timeout(_)
                | ProviderError::HttpError(_)
        ) || matches!(err, ProviderError::ApiError { status, .. } if *status >= 500)
    }
}

#[async_trait]
impl Provider for FailoverProvider {
    async fn complete(
        &self,
        request: super::types::LLMRequest,
    ) -> super::error::Result<super::types::LLMResponse> {
        let mut last_err = ProviderError::InvalidRequest("empty failover chain".to_string());
        for (i, provider) in self.chain.iter().enumerate() {
            match provider.complete(request.clone()).await {
                Ok(resp) => return Ok(resp),
                Err(e) if Self::is_failover_error(&e) => {
                    tracing::warn!(
                        "[FAILOVER] provider '{}' (index {}) failed: {}; trying next",
                        provider.name(),
                        i,
                        e
                    );
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }

    async fn stream(
        &self,
        request: super::types::LLMRequest,
    ) -> super::error::Result<super::r#trait::ProviderStream> {
        let mut last_err = ProviderError::InvalidRequest("empty failover chain".to_string());
        for (i, provider) in self.chain.iter().enumerate() {
            match provider.stream(request.clone()).await {
                Ok(s) => return Ok(s),
                Err(e) if Self::is_failover_error(&e) => {
                    tracing::warn!(
                        "[FAILOVER] provider '{}' (index {}) stream failed: {}; trying next",
                        provider.name(),
                        i,
                        e
                    );
                    last_err = e;
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_err)
    }

    fn name(&self) -> &str {
        self.chain.first().map(|p| p.name()).unwrap_or("failover")
    }

    fn default_model(&self) -> &str {
        self.chain.first().map(|p| p.default_model()).unwrap_or("")
    }

    fn supported_models(&self) -> Vec<String> {
        self.chain
            .iter()
            .flat_map(|p| p.supported_models())
            .collect()
    }

    fn context_window(&self, model: &str) -> Option<u32> {
        self.chain.first().and_then(|p| p.context_window(model))
    }

    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        self.chain
            .first()
            .map(|p| p.calculate_cost(model, input_tokens, output_tokens))
            .unwrap_or(0.0)
    }
}

/// Create a provider based on configuration with fallback priority
///
/// Priority order:
/// 1. Qwen (if configured with credentials)
/// 2. Ollama native (if `providers.ollama` is configured)
/// 3. OpenAI / OpenAI-compatible local (LM Studio, Ollama via `/v1`, LocalAI)
/// 4. Gemini (Google AI Studio - Gemini and Gemma models)
/// 5. Anthropic (default fallback)
///
/// Ollama sits between Qwen and OpenAI so that existing setups using only
/// `providers.openai.base_url` (LM Studio, Ollama-via-compat) keep resolving
/// exactly as before when `providers.ollama` is absent.
pub fn create_provider(config: &Config) -> Result<Arc<dyn Provider>> {
    // Try Qwen first
    if let Some(provider) = try_create_qwen(config)? {
        return Ok(provider);
    }

    // Try native Ollama
    if let Some(provider) = try_create_ollama(config)? {
        return Ok(provider);
    }

    // Try OpenAI
    if let Some(provider) = try_create_openai(config)? {
        return Ok(provider);
    }

    // Try Gemini (also serves Gemma models via the same API)
    if let Some(provider) = try_create_gemini(config)? {
        return Ok(provider);
    }

    // Fall back to Anthropic
    create_anthropic(config)
}

/// Try to create Gemini provider if configured and enabled.
///
/// Serves both Gemini models and Google's open-weight Gemma models (Gemma 3,
/// Gemma 4) through the same API — configuring `providers.gemini` is enough
/// to run Gemma 4 remotely without local Ollama inference.
fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    let gemini_config = match &config.providers.gemini {
        Some(cfg) if cfg.enabled => cfg,
        _ => return Ok(None),
    };

    let api_key = match &gemini_config.api_key {
        Some(key) => key.clone(),
        None => return Ok(None),
    };

    tracing::info!("Using Gemini provider");
    println!("✨ Using Google Gemini\n");

    let mut provider = match &gemini_config.base_url {
        Some(base_url) => GeminiProvider::with_base_url(api_key, base_url.clone()),
        None => GeminiProvider::new(api_key),
    };

    if let Some(model) = &gemini_config.default_model {
        tracing::info!("Using custom default model: {}", model);
        println!("📦 Model: {}\n", model);
        provider = provider.with_default_model(model.clone());
    }

    Ok(Some(Arc::new(provider)))
}

/// Try to create the native Ollama provider if `providers.ollama` is configured.
#[cfg(feature = "ollama")]
fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    use super::ollama::OllamaProvider;

    let ollama_config = match &config.providers.ollama {
        Some(cfg) if cfg.enabled => cfg,
        _ => return Ok(None),
    };

    tracing::info!("Using native Ollama at: {}", ollama_config.host);
    println!("🦙 Using native Ollama at: {}\n", ollama_config.host);

    let mut provider = OllamaProvider::new(ollama_config.host.clone());
    if let Some(model) = &ollama_config.default_model {
        tracing::info!("Using custom default model: {}", model);
        println!("📦 Model: {}\n", model);
        provider = provider.with_default_model(model.clone());
    }
    if let Some(keep_alive) = &ollama_config.keep_alive {
        provider = provider.with_keep_alive(keep_alive);
    }
    if let Some(num_ctx) = ollama_config.num_ctx {
        provider = provider.with_num_ctx(num_ctx);
    }
    provider = provider.with_sampling(
        ollama_config.temperature,
        ollama_config.top_p,
        ollama_config.top_k,
    );

    Ok(Some(Arc::new(provider)))
}

/// Without the `ollama` feature compiled in, a configured `providers.ollama`
/// section is not silently ignored - it's a clear error pointing at the
/// missing feature, rather than an unexplained fallback to another provider.
#[cfg(not(feature = "ollama"))]
fn try_create_ollama(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    if config.providers.ollama.is_some() {
        anyhow::bail!(
            "providers.ollama is configured, but this build of crustly was compiled \
             without the 'ollama' feature. Rebuild with `--features ollama` (or `all-llm`)."
        );
    }
    Ok(None)
}

/// Try to create Qwen provider if configured and enabled.
fn try_create_qwen(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    let qwen_config = match &config.providers.qwen {
        Some(cfg) if cfg.enabled => cfg,
        _ => return Ok(None),
    };

    // Local Qwen (vLLM, LM Studio, etc.)
    if let Some(base_url) = &qwen_config.base_url {
        tracing::info!("Using local Qwen at: {}", base_url);
        println!("🏠 Using local Qwen at: {}\n", base_url);

        let provider = configure_qwen(QwenProvider::local(base_url.clone()), qwen_config);
        return Ok(Some(Arc::new(provider)));
    }

    // DashScope cloud API
    if let Some(api_key) = &qwen_config.api_key {
        let region = qwen_config.region.as_deref().unwrap_or("intl");

        let provider_base = match region {
            "cn" => {
                tracing::info!("Using DashScope China (Beijing)");
                println!("☁️  Using DashScope China (Beijing)\n");
                QwenProvider::dashscope_cn(api_key.clone())
            }
            _ => {
                tracing::info!("Using DashScope International (Singapore)");
                println!("☁️  Using DashScope International (Singapore)\n");
                QwenProvider::dashscope_intl(api_key.clone())
            }
        };

        let provider = configure_qwen(provider_base, qwen_config);
        return Ok(Some(Arc::new(provider)));
    }

    Ok(None)
}

/// Configure Qwen provider with tool parser, thinking mode, and model
fn configure_qwen(mut provider: QwenProvider, config: &QwenProviderConfig) -> QwenProvider {
    // Set tool parser
    if let Some(parser) = &config.tool_parser {
        let tool_parser = match parser.as_str() {
            "openai" => ToolCallParser::OpenAI,
            "native" | "qwen" => ToolCallParser::NativeQwen,
            _ => ToolCallParser::Hermes,
        };
        provider = provider.with_tool_parser(tool_parser);
        tracing::info!("Using tool parser: {:?}", tool_parser);

        if tool_parser == ToolCallParser::NativeQwen {
            println!("🔧 Using native Qwen function calling (✿FUNCTION✿ markers)\n");
        }
    }

    // Set thinking mode
    if config.enable_thinking {
        provider = provider.with_thinking(true);
        tracing::info!("🧠 Qwen3 thinking mode enabled");
        println!("🧠 Thinking mode: enabled\n");

        if let Some(budget) = config.thinking_budget {
            provider = provider.with_thinking_budget(budget);
            tracing::info!("Thinking budget: {} tokens", budget);
        }
    }

    // Set custom model
    if let Some(model) = &config.default_model {
        tracing::info!("Using custom default model: {}", model);
        println!("📦 Model: {}\n", model);
        provider = provider.with_default_model(model.clone());
    }

    // Sampling overrides (top_p/top_k/repetition_penalty). Unset fields fall
    // back to Qwen's model-family-aware recommended defaults.
    if config.top_p.is_some() || config.top_k.is_some() || config.repetition_penalty.is_some() {
        provider = provider.with_sampling(config.top_p, config.top_k, config.repetition_penalty);
    }

    provider
}

/// Try to create OpenAI provider if configured and enabled.
fn try_create_openai(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    let openai_config = match &config.providers.openai {
        Some(cfg) if cfg.enabled => cfg,
        _ => return Ok(None),
    };

    // Local LLM (LM Studio, Ollama, etc.)
    if let Some(base_url) = &openai_config.base_url {
        tracing::info!("Using local LLM at: {}", base_url);
        println!("🏠 Using local LLM at: {}\n", base_url);

        let provider = configure_openai(OpenAIProvider::local(base_url.clone()), openai_config);
        return Ok(Some(Arc::new(provider)));
    }

    // Official OpenAI API
    if let Some(api_key) = &openai_config.api_key {
        tracing::info!("Using OpenAI provider");
        println!("🤖 Using OpenAI provider\n");

        let provider = configure_openai(OpenAIProvider::new(api_key.clone()), openai_config);
        return Ok(Some(Arc::new(provider)));
    }

    Ok(None)
}

/// Configure OpenAI provider with custom model
fn configure_openai(mut provider: OpenAIProvider, config: &ProviderConfig) -> OpenAIProvider {
    if let Some(model) = &config.default_model {
        tracing::info!("Using custom default model: {}", model);
        println!("📦 Model: {}\n", model);
        provider = provider.with_default_model(model.clone());
    }
    provider
}

/// Create Anthropic provider (default fallback)
fn create_anthropic(config: &Config) -> Result<Arc<dyn Provider>> {
    let anthropic_config = config.providers.anthropic.as_ref().context(
        "No provider configured.\n\nPlease set one of:\n  - ANTHROPIC_API_KEY for Claude\n  - OPENAI_API_KEY for OpenAI/GPT\n  - OPENAI_BASE_URL for local LLMs (LM Studio, Ollama)\n  - GEMINI_API_KEY for Google Gemini/Gemma\n  - QWEN_BASE_URL for local Qwen (vLLM)\n  - DASHSCOPE_API_KEY for DashScope cloud\n\nExample for vLLM with Qwen:\n  export QWEN_BASE_URL=\"http://localhost:8000/v1/chat/completions\"",
    )?;

    // Anthropic is the terminal fallback, so a disabled one cannot quietly fall
    // through to another provider the way the others can - it must say so, rather
    // than fail later with a misleading "API key not set".
    if !anthropic_config.enabled {
        anyhow::bail!(
            "No provider is enabled. `providers.anthropic` is the last fallback and it \
             has `enabled = false`.\n\nEnable a provider in config.toml, or remove its \
             `enabled = false`."
        );
    }

    let api_key = anthropic_config
        .api_key
        .as_ref()
        .context("Anthropic API key not set")?
        .clone();

    tracing::info!("Using Anthropic provider");
    println!("🤖 Using Anthropic Claude\n");

    Ok(Arc::new(AnthropicProvider::new(api_key)))
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{ProviderError, Result as ProviderResult},
        r#trait::ProviderStream,
        types::{ContentBlock, LLMRequest, LLMResponse, StopReason, TokenUsage},
    };
    use super::*;
    use crate::config::{Config, ProviderConfig, ProviderConfigs, QwenProviderConfig};
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // ── Mock providers for T063 ────────────────────────────────────────────────

    /// Always fails with a rate-limit error.
    struct RateLimitedProvider;

    #[async_trait]
    impl Provider for RateLimitedProvider {
        async fn complete(&self, _req: LLMRequest) -> ProviderResult<LLMResponse> {
            Err(ProviderError::RateLimitExceeded("mock 429".to_string()))
        }
        async fn stream(&self, _req: LLMRequest) -> ProviderResult<ProviderStream> {
            Err(ProviderError::RateLimitExceeded("mock 429".to_string()))
        }
        fn name(&self) -> &str {
            "mock-primary"
        }
        fn default_model(&self) -> &str {
            "mock"
        }
        fn supported_models(&self) -> Vec<String> {
            vec!["mock".to_string()]
        }
        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }
        fn calculate_cost(&self, _model: &str, _in: u32, _out: u32) -> f64 {
            0.0
        }
    }

    /// Succeeds and increments a call counter.
    struct SucceedingProvider {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl Provider for SucceedingProvider {
        async fn complete(&self, _req: LLMRequest) -> ProviderResult<LLMResponse> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(LLMResponse {
                id: "test-id".to_string(),
                model: "mock-secondary".to_string(),
                content: vec![ContentBlock::Text {
                    text: "ok".to_string(),
                }],
                stop_reason: Some(StopReason::EndTurn),
                usage: TokenUsage {
                    input_tokens: 1,
                    output_tokens: 1,
                },
                cache_metrics: None,
                perf_metrics: None,
            })
        }
        async fn stream(&self, _req: LLMRequest) -> ProviderResult<ProviderStream> {
            Err(ProviderError::InvalidRequest(
                "streaming not implemented in mock".to_string(),
            ))
        }
        fn name(&self) -> &str {
            "mock-secondary"
        }
        fn default_model(&self) -> &str {
            "mock"
        }
        fn supported_models(&self) -> Vec<String> {
            vec!["mock".to_string()]
        }
        fn context_window(&self, _model: &str) -> Option<u32> {
            Some(4096)
        }
        fn calculate_cost(&self, _model: &str, _in: u32, _out: u32) -> f64 {
            0.0
        }
    }

    #[tokio::test]
    async fn test_failover_on_rate_limit_tries_next_provider() {
        let counter = Arc::new(AtomicUsize::new(0));
        let primary: Arc<dyn Provider> = Arc::new(RateLimitedProvider);
        let secondary: Arc<dyn Provider> = Arc::new(SucceedingProvider {
            calls: counter.clone(),
        });

        let failover = FailoverProvider::new(vec![primary, secondary]);
        let req = LLMRequest::new("mock", vec![]);

        let result = failover.complete(req).await;

        assert!(result.is_ok(), "failover should succeed via secondary");
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "secondary must be called once"
        );
    }

    #[tokio::test]
    async fn test_failover_all_fail_returns_last_error() {
        let p1: Arc<dyn Provider> = Arc::new(RateLimitedProvider);
        let p2: Arc<dyn Provider> = Arc::new(RateLimitedProvider);
        let failover = FailoverProvider::new(vec![p1, p2]);

        let result = failover.complete(LLMRequest::new("mock", vec![])).await;
        assert!(
            result.is_err(),
            "should propagate error when all providers fail"
        );
        let err = result.unwrap_err();
        assert!(matches!(err, ProviderError::RateLimitExceeded(_)));
    }

    #[test]
    fn test_create_provider_with_anthropic() {
        let config = Config {
            providers: ProviderConfigs {
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = create_provider(&config);
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.name(), "anthropic");
    }

    #[test]
    fn test_create_provider_with_openai() {
        let config = Config {
            providers: ProviderConfigs {
                openai: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = create_provider(&config);
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_create_provider_with_gemini() {
        let config = Config {
            providers: ProviderConfigs {
                gemini: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("gemini should be selected");
        assert_eq!(provider.name(), "gemini");
        assert_eq!(provider.default_model(), "gemini-2.5-flash");
    }

    #[test]
    fn test_create_provider_with_gemini_custom_base_url_and_model() {
        let config = Config {
            providers: ProviderConfigs {
                gemini: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("test-key".to_string()),
                    base_url: Some("https://gemini-proxy.internal/v1beta".to_string()),
                    default_model: Some("gemma-4-31b-it".to_string()),
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("gemini should be selected");
        assert_eq!(provider.name(), "gemini");
        assert_eq!(provider.default_model(), "gemma-4-31b-it");
    }

    #[test]
    fn gemini_without_api_key_falls_through_to_anthropic() {
        let config = Config {
            providers: ProviderConfigs {
                gemini: Some(ProviderConfig {
                    enabled: true,
                    api_key: None,
                    base_url: None,
                    default_model: None,
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("anthropic should be selected");
        assert_eq!(
            provider.name(),
            "anthropic",
            "gemini with no api_key must not be selected",
        );
    }

    #[test]
    fn disabled_gemini_is_skipped_in_favour_of_the_next_provider() {
        let config = Config {
            providers: ProviderConfigs {
                gemini: Some(ProviderConfig {
                    enabled: false,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("anthropic should be selected");
        assert_eq!(
            provider.name(),
            "anthropic",
            "a disabled Gemini must not be selected despite having an api_key",
        );
    }

    /// Regression: `enabled = false` was read from config but never checked by the
    /// Qwen, OpenAI or Anthropic branches - only Ollama honoured it. A provider the
    /// user had explicitly turned off would still be selected, purely on the
    /// presence of an api_key/base_url, and would silently take over as soon as the
    /// provider above it in the priority order was removed.
    #[test]
    fn disabled_openai_is_skipped_in_favour_of_the_next_provider() {
        let config = Config {
            providers: ProviderConfigs {
                openai: Some(ProviderConfig {
                    enabled: false,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("anthropic should be selected");
        assert_eq!(
            provider.name(),
            "anthropic",
            "a disabled OpenAI must not be selected despite having an api_key",
        );
    }

    #[test]
    fn disabled_qwen_is_skipped_in_favour_of_the_next_provider() {
        let config = Config {
            providers: ProviderConfigs {
                qwen: Some(QwenProviderConfig {
                    enabled: false,
                    base_url: Some("http://localhost:8000/v1".to_string()),
                    ..Default::default()
                }),
                anthropic: Some(ProviderConfig {
                    enabled: true,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let provider = create_provider(&config).expect("anthropic should be selected");
        assert_eq!(
            provider.name(),
            "anthropic",
            "a disabled Qwen must not be selected despite having a base_url",
        );
    }

    /// Anthropic is the terminal fallback: disabling it cannot fall through to
    /// anything, so it must fail with an explanation rather than the misleading
    /// "Anthropic API key not set" it would otherwise hit.
    #[test]
    fn disabled_anthropic_fallback_fails_with_a_clear_message() {
        let config = Config {
            providers: ProviderConfigs {
                anthropic: Some(ProviderConfig {
                    enabled: false,
                    api_key: Some("anthropic-key".to_string()),
                    base_url: None,
                    default_model: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        // `Arc<dyn Provider>` is not Debug, so `expect_err` is unavailable here.
        let msg = match create_provider(&config) {
            Ok(p) => panic!("expected an error, but got provider {:?}", p.name()),
            Err(e) => e.to_string(),
        };
        assert!(
            msg.contains("No provider is enabled"),
            "expected an explanation that nothing is enabled, got: {msg}"
        );
    }

    #[test]
    fn test_create_provider_with_qwen() {
        let config = Config {
            providers: ProviderConfigs {
                qwen: Some(QwenProviderConfig {
                    enabled: true,
                    api_key: Some("test-key".to_string()),
                    base_url: None,
                    default_model: None,
                    tool_parser: None,
                    enable_thinking: false,
                    thinking_budget: None,
                    region: None,
                    top_p: None,
                    top_k: None,
                    repetition_penalty: None,
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let result = create_provider(&config);
        assert!(result.is_ok());
        let provider = result.unwrap();
        assert_eq!(provider.name(), "qwen");
    }

    #[test]
    fn test_create_provider_no_credentials() {
        let config = Config {
            providers: ProviderConfigs {
                anthropic: None,
                openai: None,
                qwen: None,
                ..Default::default()
            },
            ..Default::default()
        };

        let result = create_provider(&config);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("No provider configured"));
        }
    }
}

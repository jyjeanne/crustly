# LLM Provider Implementation Guide

This guide explains how to implement new LLM providers for Crustly.

## Overview

All providers must implement the `Provider` trait defined in `src/llm/provider/trait.rs`.

## Provider Trait

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn default_model(&self) -> &str;
    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse>;
    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream>;
    fn supported_models(&self) -> Vec<String>;
    fn context_window(&self, model: &str) -> Option<u32>;
    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64;

    // Optional overrides
    fn supports_streaming(&self) -> bool { true }
    fn supports_tools(&self) -> bool { true }
    fn supports_vision(&self) -> bool { false }
    fn validate_model(&self, model: &str) -> bool { ... }
}
```

## Implementation Steps

### 1. Create Provider File

Create a new file in `src/llm/provider/` (e.g., `gemini.rs`):

```rust
//! Google Gemini Provider
//!
//! Implementation for Google's Gemini API.

use super::{Provider, ProviderStream, Result};
use super::{LLMRequest, LLMResponse, ContentBlock, Role, Tool};
use super::error::ProviderError;
use async_trait::async_trait;
use reqwest::Client;

pub struct GeminiProvider {
    api_key: String,
    client: Client,
    custom_default_model: Option<String>,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            api_key,
            client,
            custom_default_model: None,
        }
    }

    pub fn with_default_model(mut self, model: String) -> Self {
        self.custom_default_model = Some(model);
        self
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    fn default_model(&self) -> &str {
        self.custom_default_model.as_deref().unwrap_or("gemini-1.5-flash")
    }

    async fn complete(&self, request: LLMRequest) -> Result<LLMResponse> {
        // Implement API call
        todo!()
    }

    async fn stream(&self, request: LLMRequest) -> Result<ProviderStream> {
        // Implement streaming
        todo!()
    }

    fn supported_models(&self) -> Vec<String> {
        vec![
            "gemini-1.5-pro".to_string(),
            "gemini-1.5-flash".to_string(),
        ]
    }

    fn context_window(&self, model: &str) -> Option<u32> {
        Some(match model {
            "gemini-1.5-pro" => 2_000_000,
            "gemini-1.5-flash" => 1_000_000,
            _ => 32_768,
        })
    }

    fn calculate_cost(&self, model: &str, input_tokens: u32, output_tokens: u32) -> f64 {
        let (input_price, output_price) = match model {
            "gemini-1.5-pro" => (0.00125, 0.005),
            "gemini-1.5-flash" => (0.000075, 0.0003),
            _ => (0.0005, 0.0015),
        };

        (input_tokens as f64 / 1000.0) * input_price +
        (output_tokens as f64 / 1000.0) * output_price
    }
}
```

### 2. Register in Module

Add to `src/llm/provider/mod.rs`:

```rust
pub mod gemini;
pub use gemini::GeminiProvider;
```

### 3. Add to Factory

Update `src/llm/provider/factory.rs` to include your provider in the creation logic:

```rust
pub fn create_provider(config: &Config) -> Result<Arc<dyn Provider>> {
    // Try Gemini first (if you want it prioritized)
    if let Some(provider) = try_create_gemini(config)? {
        return Ok(provider);
    }

    // ... existing logic
}

fn try_create_gemini(config: &Config) -> Result<Option<Arc<dyn Provider>>> {
    let gemini_config = match &config.providers.gemini {
        Some(cfg) if cfg.enabled => cfg,
        _ => return Ok(None),
    };

    if let Some(api_key) = &gemini_config.api_key {
        let provider = GeminiProvider::new(api_key.clone());
        return Ok(Some(Arc::new(provider)));
    }

    Ok(None)
}
```

### 4. Add Configuration Support

Update `src/config/mod.rs` to add the provider config if not already present:

```rust
pub struct ProviderConfigs {
    pub gemini: Option<ProviderConfig>,
    // ... other providers
}
```

### 5. Update Keyring Support

Add to `src/config/secrets.rs`:

```rust
pub fn load_with_fallback() -> Self {
    Self {
        gemini: SecretString::load_with_fallback("gemini_api_key", "GEMINI_API_KEY"),
        // ... other providers
    }
}
```

## Type Mappings

### Request Types

```rust
pub struct LLMRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub system: Option<String>,
    pub tools: Option<Vec<Tool>>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub stream: bool,
}

pub struct Message {
    pub role: Role,  // User, Assistant, System
    pub content: Vec<ContentBlock>,
}

pub enum ContentBlock {
    Text { text: String },
    Image { source: ImageSource },
    ToolUse { id: String, name: String, input: serde_json::Value },
    ToolResult { tool_use_id: String, content: String, is_error: Option<bool> },
}
```

### Response Types

```rust
pub struct LLMResponse {
    pub id: String,
    pub model: String,
    pub content: Vec<ContentBlock>,
    pub stop_reason: Option<StopReason>,
    pub usage: TokenUsage,
}

pub enum StopReason {
    EndTurn,
    MaxTokens,
    StopSequence,
    ToolUse,
}

pub struct TokenUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}
```

## Error Handling

Use `ProviderError` for all errors:

```rust
use super::error::ProviderError;

// Network errors
Err(ProviderError::HttpError(e))

// API errors
Err(ProviderError::ApiError {
    status: 400,
    message: "Bad request".to_string(),
    error_type: Some("invalid_request".to_string()),
})

// Rate limiting
Err(ProviderError::RateLimitExceeded("Retry after 60s".to_string()))
```

## Testing

Add comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = GeminiProvider::new("test-key".to_string());
        assert_eq!(provider.name(), "gemini");
    }

    #[test]
    fn test_context_window() {
        let provider = GeminiProvider::new("test-key".to_string());
        assert_eq!(provider.context_window("gemini-1.5-pro"), Some(2_000_000));
    }

    #[test]
    fn test_cost_calculation() {
        let provider = GeminiProvider::new("test-key".to_string());
        let cost = provider.calculate_cost("gemini-1.5-flash", 1000, 1000);
        assert!((cost - 0.000375).abs() < 0.000001);
    }

    #[test]
    fn test_supported_models() {
        let provider = GeminiProvider::new("test-key".to_string());
        let models = provider.supported_models();
        assert!(models.contains(&"gemini-1.5-pro".to_string()));
    }
}
```

## Existing Provider Examples

### Simple Provider: Azure OpenAI
- File: `src/llm/provider/azure.rs`
- Wraps OpenAI provider for Azure endpoints
- Good example of provider composition

### Complex Provider: Qwen
- File: `src/llm/provider/qwen.rs`
- Multiple tool call parsers
- Thinking mode support
- Local and cloud variants

### Standard Provider: Anthropic
- File: `src/llm/provider/anthropic.rs`
- Streaming implementation
- Tool calling
- Good reference implementation

## Common Patterns

### HTTP Client Setup
```rust
let client = Client::builder()
    .timeout(Duration::from_secs(120))
    .connect_timeout(Duration::from_secs(10))
    .pool_idle_timeout(Duration::from_secs(90))
    .build()
    .expect("Failed to create HTTP client");
```

### Role Conversion
```rust
fn convert_role(role: &Role) -> &str {
    match role {
        Role::User => "user",
        Role::Assistant => "assistant",  // or "model" for Gemini
        Role::System => "system",
    }
}
```

### Content Block Handling
```rust
for block in &message.content {
    match block {
        ContentBlock::Text { text } => {
            // Handle text
        }
        ContentBlock::ToolUse { id, name, input } => {
            // Handle tool use
        }
        ContentBlock::ToolResult { tool_use_id, content, .. } => {
            // Handle tool result
        }
        _ => {}
    }
}
```

## Providers Needed

Priority order for implementation:

1. **Google Gemini** - Popular, vision support, cheap
2. **AWS Bedrock** - Enterprise use, multi-model
3. **Google VertexAI** - Enterprise Gemini
4. **Cohere** - Good for embeddings
5. **Mistral AI** - European alternative

## Resources

- Anthropic SDK: https://github.com/anthropics/anthropic-sdk-rust
- OpenAI API: https://platform.openai.com/docs/api-reference
- Gemini API: https://ai.google.dev/docs
- Bedrock API: https://docs.aws.amazon.com/bedrock/

## Contributing

When implementing a provider:
1. Follow the existing pattern (see Azure or Anthropic)
2. Add comprehensive tests (4+ test cases minimum)
3. Update factory.rs to include the provider
4. Add configuration support
5. Update README with the new provider
6. Document any provider-specific quirks

---

**Note:** Provider implementations are welcome contributions! If you implement a provider, please submit a PR with tests and documentation.

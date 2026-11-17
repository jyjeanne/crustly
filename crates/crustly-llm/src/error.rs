//! Error types for LLM providers

use thiserror::Error;

/// Provider error types
#[derive(Debug, Error)]
pub enum ProviderError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// API returned an error
    #[error("API error ({status}): {message}")]
    ApiError {
        status: u16,
        message: String,
        error_type: Option<String>,
    },

    /// Invalid API key
    #[error("Invalid API key")]
    InvalidApiKey,

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Context length exceeded
    #[error("Context length exceeded: {0} tokens")]
    ContextLengthExceeded(u32),

    /// Streaming not supported
    #[error("Streaming not supported by this provider")]
    StreamingNotSupported,

    /// Tools not supported
    #[error("Tools not supported by this provider")]
    ToolsNotSupported,

    /// JSON parsing error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Streaming error
    #[error("Streaming error: {0}")]
    StreamError(String),

    /// Timeout
    #[error("Request timed out after {0}s")]
    Timeout(u64),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ProviderError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ProviderError::HttpError(_)
            | ProviderError::RateLimitExceeded(_)
            | ProviderError::Timeout(_) => true,
            ProviderError::ApiError { status, .. } if *status >= 500 => true,
            _ => false,
        }
    }

    /// Whether the provider rejected the request because the prompt did not
    /// fit the model's context window. Besides [`Self::ContextLengthExceeded`]
    /// (in-process llama.cpp), recognises the wording of OpenAI-compatible
    /// servers (OpenAI, vLLM, llama.cpp server, LM Studio), Anthropic and
    /// Gemini, which report an overflow as an ordinary API error.
    pub fn is_context_overflow(&self) -> bool {
        match self {
            ProviderError::ContextLengthExceeded(_) => true,
            ProviderError::ApiError {
                message,
                error_type,
                ..
            } => {
                mentions_context_overflow(message)
                    || error_type.as_deref().is_some_and(mentions_context_overflow)
            }
            ProviderError::InvalidRequest(message) => mentions_context_overflow(message),
            _ => false,
        }
    }

    /// Get HTTP status code if available
    pub fn status_code(&self) -> Option<u16> {
        match self {
            ProviderError::ApiError { status, .. } => Some(*status),
            _ => None,
        }
    }
}

/// Whether an error text describes a prompt that did not fit the context window.
fn mentions_context_overflow(text: &str) -> bool {
    const PHRASES: &[&str] = &[
        "context length",
        "context_length",
        "maximum context",
        "context window",
        "prompt is too long",
        "too many tokens",
        "exceeds the available context",
        "exceeds the maximum number of tokens",
    ];
    let lower = text.to_lowercase();
    PHRASES.iter().any(|p| lower.contains(p))
}

/// Result type for provider operations
pub type Result<T> = std::result::Result<T, ProviderError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_context_overflow_from_each_provider() {
        let api = |message: &str| ProviderError::ApiError {
            status: 400,
            message: message.to_string(),
            error_type: None,
        };
        assert!(ProviderError::ContextLengthExceeded(9000).is_context_overflow());
        // OpenAI / vLLM
        assert!(api("This model's maximum context length is 8192 tokens").is_context_overflow());
        // Anthropic
        assert!(api("prompt is too long: 210000 tokens > 200000 maximum").is_context_overflow());
        // llama.cpp server / LM Studio
        assert!(api("the request exceeds the available context size").is_context_overflow());
        // Gemini
        assert!(
            api("The input token count (40000) exceeds the maximum number of tokens allowed")
                .is_context_overflow()
        );
        assert!(ProviderError::ApiError {
            status: 400,
            message: "bad request".to_string(),
            error_type: Some("context_length_exceeded".to_string()),
        }
        .is_context_overflow());

        assert!(!api("Bad Request").is_context_overflow());
        assert!(!ProviderError::InvalidApiKey.is_context_overflow());
        assert!(!ProviderError::Timeout(30).is_context_overflow());
    }

    #[test]
    fn test_error_retryable() {
        let rate_limit = ProviderError::RateLimitExceeded("Try again later".to_string());
        assert!(rate_limit.is_retryable());

        let invalid_key = ProviderError::InvalidApiKey;
        assert!(!invalid_key.is_retryable());

        let server_error = ProviderError::ApiError {
            status: 500,
            message: "Internal Server Error".to_string(),
            error_type: None,
        };
        assert!(server_error.is_retryable());

        let client_error = ProviderError::ApiError {
            status: 400,
            message: "Bad Request".to_string(),
            error_type: None,
        };
        assert!(!client_error.is_retryable());
    }

    #[test]
    fn test_status_code() {
        let error = ProviderError::ApiError {
            status: 429,
            message: "Too many requests".to_string(),
            error_type: Some("rate_limit_error".to_string()),
        };
        assert_eq!(error.status_code(), Some(429));

        let invalid_key = ProviderError::InvalidApiKey;
        assert_eq!(invalid_key.status_code(), None);
    }
}

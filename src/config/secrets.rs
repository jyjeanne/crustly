//! Secure secret management
//!
//! This module provides secure handling of sensitive data like API keys,
//! ensuring they are properly zeroized from memory when dropped.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A secure string that zeroizes its contents on drop
///
/// This type should be used for any sensitive data like API keys,
/// passwords, or tokens to ensure they are properly cleared from memory.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecretString {
    inner: String,
}

impl SecretString {
    /// Create a new SecretString from a String
    pub fn new(value: String) -> Self {
        Self { inner: value }
    }

    /// Create a new SecretString from a &str
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }

    /// Load a secret from an environment variable
    pub fn from_env(var_name: &str) -> Result<Self> {
        let value = std::env::var(var_name)
            .with_context(|| format!("Environment variable not found: {}", var_name))?;
        Ok(Self::new(value))
    }

    /// Load an optional secret from an environment variable
    pub fn from_env_optional(var_name: &str) -> Option<Self> {
        std::env::var(var_name).ok().map(Self::new)
    }

    /// Get a reference to the inner string
    ///
    /// # Security Warning
    /// Use with caution! This exposes the sensitive data.
    /// Avoid logging or displaying the returned value.
    pub fn expose_secret(&self) -> &str {
        &self.inner
    }

    /// Check if the secret is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get the length of the secret
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl fmt::Debug for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

// Custom Serialize implementation to prevent accidental serialization
impl Serialize for SecretString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Never serialize the actual secret
        serializer.serialize_str("[REDACTED]")
    }
}

// Custom Deserialize implementation
impl<'de> Deserialize<'de> for SecretString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SecretString::new(s))
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        SecretString::new(s)
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        SecretString::from_str(s)
    }
}

/// Provider API keys collection
///
/// Stores API keys for various LLM providers with secure zeroization.
#[derive(Debug, Clone)]
pub struct ProviderSecrets {
    pub anthropic: Option<SecretString>,
    pub openai: Option<SecretString>,
    pub gemini: Option<SecretString>,
    pub azure: Option<SecretString>,
}

impl ProviderSecrets {
    /// Create a new empty ProviderSecrets
    pub fn new() -> Self {
        Self {
            anthropic: None,
            openai: None,
            gemini: None,
            azure: None,
        }
    }

    /// Load all provider secrets from environment variables
    pub fn from_env() -> Self {
        Self {
            anthropic: SecretString::from_env_optional("ANTHROPIC_API_KEY"),
            openai: SecretString::from_env_optional("OPENAI_API_KEY"),
            gemini: SecretString::from_env_optional("GEMINI_API_KEY"),
            azure: SecretString::from_env_optional("AZURE_OPENAI_KEY"),
        }
    }

    /// Check if any secrets are configured
    pub fn has_any(&self) -> bool {
        self.anthropic.is_some()
            || self.openai.is_some()
            || self.gemini.is_some()
            || self.azure.is_some()
    }

    /// Get the number of configured secrets
    pub fn count(&self) -> usize {
        let mut count = 0;
        if self.anthropic.is_some() {
            count += 1;
        }
        if self.openai.is_some() {
            count += 1;
        }
        if self.gemini.is_some() {
            count += 1;
        }
        if self.azure.is_some() {
            count += 1;
        }
        count
    }
}

impl Default for ProviderSecrets {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_string_creation() {
        let secret = SecretString::from_str("my-secret-key");
        assert_eq!(secret.expose_secret(), "my-secret-key");
        assert_eq!(secret.len(), 13);
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_secret_string_debug() {
        let secret = SecretString::from_str("my-secret-key");
        let debug_output = format!("{:?}", secret);
        assert_eq!(debug_output, "[REDACTED]");
        assert!(!debug_output.contains("my-secret-key"));
    }

    #[test]
    fn test_secret_string_display() {
        let secret = SecretString::from_str("my-secret-key");
        let display_output = format!("{}", secret);
        assert_eq!(display_output, "[REDACTED]");
        assert!(!display_output.contains("my-secret-key"));
    }

    #[test]
    fn test_provider_secrets_empty() {
        let secrets = ProviderSecrets::new();
        assert!(!secrets.has_any());
        assert_eq!(secrets.count(), 0);
    }

    #[test]
    fn test_provider_secrets_with_keys() {
        let secrets = ProviderSecrets {
            anthropic: Some(SecretString::from_str("key1")),
            openai: Some(SecretString::from_str("key2")),
            gemini: None,
            azure: None,
        };
        assert!(secrets.has_any());
        assert_eq!(secrets.count(), 2);
    }

    #[test]
    fn test_secret_string_from_env() {
        // Set a test environment variable
        std::env::set_var("TEST_SECRET_KEY", "test-value");

        let secret = SecretString::from_env("TEST_SECRET_KEY").unwrap();
        assert_eq!(secret.expose_secret(), "test-value");

        // Clean up
        std::env::remove_var("TEST_SECRET_KEY");
    }

    #[test]
    fn test_secret_string_from_env_optional() {
        // Test with non-existent variable
        let secret = SecretString::from_env_optional("NONEXISTENT_KEY");
        assert!(secret.is_none());

        // Test with existing variable
        std::env::set_var("TEST_OPTIONAL_KEY", "optional-value");
        let secret = SecretString::from_env_optional("TEST_OPTIONAL_KEY");
        assert!(secret.is_some());
        assert_eq!(secret.unwrap().expose_secret(), "optional-value");

        // Clean up
        std::env::remove_var("TEST_OPTIONAL_KEY");
    }

    #[test]
    fn test_secret_string_serialize() {
        let secret = SecretString::from_str("my-secret-key");
        let serialized = serde_json::to_string(&secret).unwrap();
        assert_eq!(serialized, "\"[REDACTED]\"");
        assert!(!serialized.contains("my-secret-key"));
    }
}

//! Web Fetch Tool
//!
//! Fetch the content of any public URL and return it as text.
//! Distinct from web_search (which queries DuckDuckGo) — this directly retrieves
//! a known URL: docs pages, GitHub raw files, API endpoints, etc.

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Compile regexes once at startup rather than on every call to html_to_text.
static SCRIPT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?si)<script[^>]*>.*?</script>").unwrap());
static STYLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?si)<style[^>]*>.*?</style>").unwrap());
static TAG_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<[^>]+>").unwrap());
static WS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[ \t]+").unwrap());
static NEWLINE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n{3,}").unwrap());

/// Web fetch tool — GET a URL and return its text content
pub struct WebFetchTool;

#[derive(Debug, Deserialize, Serialize)]
struct WebFetchInput {
    /// URL to fetch
    url: String,

    /// Timeout in seconds (default: 30, max: 120)
    #[serde(default = "default_timeout")]
    timeout_secs: u64,

    /// Maximum bytes to return (default: 500 000)
    #[serde(default = "default_max_bytes")]
    max_bytes: usize,

    /// Strip HTML tags and return plain text (default: true)
    #[serde(default = "default_true")]
    strip_html: bool,
}

fn default_timeout() -> u64 {
    30
}
fn default_max_bytes() -> usize {
    500_000
}
fn default_true() -> bool {
    true
}

/// Very lightweight HTML → plain-text conversion using regexes.
/// Strips tags, collapses whitespace, and decodes a handful of common entities.
fn html_to_text(html: &str) -> String {
    // Remove <script> and <style> blocks entirely (content is not useful text).
    // The Rust regex crate does not support backreferences, so use two separate patterns.
    let text = SCRIPT_RE.replace_all(html, " ");
    let text = STYLE_RE.replace_all(&text, " ");

    // Remove all remaining tags
    let text = TAG_RE.replace_all(&text, " ");

    // Decode common HTML entities
    let text = text
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ")
        .replace("&apos;", "'");

    // Collapse whitespace runs into single spaces / newlines
    let text = WS_RE.replace_all(&text, " ");
    NEWLINE_RE.replace_all(&text, "\n\n").trim().to_string()
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "web_fetch"
    }

    fn description(&self) -> &str {
        "Fetch the content of a URL and return it as text. Use this to read documentation pages, \
         GitHub raw files, API responses, or any public HTTP/HTTPS resource. \
         HTML is stripped to plain text by default."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "URL to fetch (must start with http:// or https://)"
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Request timeout in seconds (default: 30, max: 120)",
                    "default": 30,
                    "minimum": 1,
                    "maximum": 120
                },
                "max_bytes": {
                    "type": "integer",
                    "description": "Maximum bytes to return (default: 500000, max: 5000000)",
                    "default": 500000,
                    "minimum": 1024,
                    "maximum": 5000000
                },
                "strip_html": {
                    "type": "boolean",
                    "description": "Strip HTML tags and return plain text (default: true)",
                    "default": true
                }
            },
            "required": ["url"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::Network]
    }

    fn requires_approval(&self) -> bool {
        false // Read-only fetch
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: WebFetchInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        if !input.url.starts_with("http://") && !input.url.starts_with("https://") {
            return Err(ToolError::InvalidInput(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        if input.timeout_secs == 0 || input.timeout_secs > 120 {
            return Err(ToolError::InvalidInput(
                "timeout_secs must be between 1 and 120".to_string(),
            ));
        }

        if input.max_bytes < 1024 || input.max_bytes > 5_000_000 {
            return Err(ToolError::InvalidInput(
                "max_bytes must be between 1024 and 5000000".to_string(),
            ));
        }

        Ok(())
    }

    async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: WebFetchInput = serde_json::from_value(input)?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(input.timeout_secs))
            .user_agent("crustly/0.4 (https://github.com/jyjeanne/crustly)")
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| ToolError::Execution(format!("Failed to build HTTP client: {}", e)))?;

        let response = client.get(&input.url).send().await.map_err(|e| {
            if e.is_timeout() {
                ToolError::Timeout(input.timeout_secs)
            } else if e.is_connect() {
                ToolError::Execution(format!("Connection failed: {}", e))
            } else {
                ToolError::Execution(format!("Request failed: {}", e))
            }
        })?;

        let status = response.status();
        if !status.is_success() {
            return Ok(ToolResult::error(format!(
                "HTTP {} {} for URL: {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or(""),
                input.url
            )));
        }

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let bytes = response
            .bytes()
            .await
            .map_err(|e| ToolError::Execution(format!("Failed to read response body: {}", e)))?;

        let total_bytes = bytes.len();
        let capped = bytes.len().min(input.max_bytes);
        let raw = String::from_utf8_lossy(&bytes[..capped]).into_owned();

        let is_html = content_type.contains("text/html")
            || raw.trim_start().starts_with("<!DOCTYPE")
            || raw.trim_start().starts_with("<html");

        let body = if input.strip_html && is_html {
            html_to_text(&raw)
        } else {
            raw
        };

        let mut output = format!("URL: {}\nContent-Type: {}\n\n", input.url, content_type);
        output.push_str(&body);
        if total_bytes > input.max_bytes {
            output.push_str(&format!(
                "\n\n[truncated — {} bytes total, showing first {}]",
                total_bytes, input.max_bytes
            ));
        }

        Ok(ToolResult::success(output)
            .with_metadata("url".to_string(), input.url)
            .with_metadata("content_type".to_string(), content_type)
            .with_metadata("size_bytes".to_string(), total_bytes.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_to_text_strips_tags() {
        let html = "<html><body><h1>Hello</h1><p>World</p></body></html>";
        let text = html_to_text(html);
        assert!(text.contains("Hello"));
        assert!(text.contains("World"));
        assert!(!text.contains('<'));
    }

    #[test]
    fn test_html_to_text_strips_script() {
        let html = "<p>Visible</p><script>alert('xss')</script><p>Also visible</p>";
        let text = html_to_text(html);
        assert!(text.contains("Visible"));
        assert!(text.contains("Also visible"));
        assert!(!text.contains("alert"));
    }

    #[test]
    fn test_html_to_text_decodes_entities() {
        let html = "<p>a &amp; b &lt;c&gt;</p>";
        let text = html_to_text(html);
        assert!(text.contains("a & b <c>"));
    }

    #[test]
    fn test_validate_input_rejects_non_http() {
        let tool = WebFetchTool;
        let result = tool.validate_input(&serde_json::json!({ "url": "ftp://example.com" }));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_input_accepts_https() {
        let tool = WebFetchTool;
        let result = tool.validate_input(&serde_json::json!({ "url": "https://example.com" }));
        assert!(result.is_ok());
    }
}

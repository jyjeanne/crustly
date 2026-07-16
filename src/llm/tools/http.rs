//! HTTP Client Tool
//!
//! Make HTTP requests to external APIs (REST endpoints, webhooks, etc.)

use super::error::{Result, ToolError};
use super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult};
use async_trait::async_trait;
use reqwest::{header::HeaderMap, Client, Method};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration as StdDuration;

/// HTTP client tool for external API integration
pub struct HttpClientTool;

#[derive(Debug, Deserialize, Serialize)]
struct HttpInput {
    /// HTTP method
    method: String,

    /// URL to request
    url: String,

    /// Optional: Request headers
    #[serde(default)]
    headers: HashMap<String, String>,

    /// Optional: Request body (for POST, PUT, PATCH)
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Value>,

    /// Optional: Query parameters
    #[serde(default)]
    query: HashMap<String, String>,

    /// Optional: Timeout in seconds (default: 30, max: 120)
    #[serde(default = "default_timeout")]
    timeout_secs: u64,

    /// Optional: Follow redirects (default: true)
    #[serde(default = "default_true")]
    follow_redirects: bool,
}

fn default_timeout() -> u64 {
    30
}

fn default_true() -> bool {
    true
}

fn parse_method(method_str: &str) -> Result<Method> {
    match method_str.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "PATCH" => Ok(Method::PATCH),
        "DELETE" => Ok(Method::DELETE),
        "HEAD" => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        _ => Err(ToolError::InvalidInput(format!(
            "Unsupported HTTP method: {}. Supported: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS",
            method_str
        ))),
    }
}

#[async_trait]
impl Tool for HttpClientTool {
    fn name(&self) -> &str {
        "http_request"
    }

    fn description(&self) -> &str {
        "Make HTTP requests to external APIs. Supports GET, POST, PUT, PATCH, DELETE methods with headers, query parameters, and JSON bodies. Useful for integrating with GitHub, Slack, Jira, databases, and other web services."
    }

    fn input_schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "method": {
                    "type": "string",
                    "description": "HTTP method",
                    "enum": ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]
                },
                "url": {
                    "type": "string",
                    "description": "URL to request (must be valid HTTP/HTTPS URL)"
                },
                "headers": {
                    "type": "object",
                    "description": "Request headers as key-value pairs",
                    "additionalProperties": {
                        "type": "string"
                    },
                    "default": {}
                },
                "body": {
                    "description": "Request body (JSON, for POST/PUT/PATCH)"
                },
                "query": {
                    "type": "object",
                    "description": "Query parameters as key-value pairs",
                    "additionalProperties": {
                        "type": "string"
                    },
                    "default": {}
                },
                "timeout_secs": {
                    "type": "integer",
                    "description": "Request timeout in seconds (default: 30, max: 120)",
                    "default": 30,
                    "minimum": 1,
                    "maximum": 120
                },
                "follow_redirects": {
                    "type": "boolean",
                    "description": "Follow HTTP redirects (default: true)",
                    "default": true
                }
            },
            "required": ["method", "url"]
        })
    }

    fn capabilities(&self) -> Vec<ToolCapability> {
        vec![ToolCapability::Network]
    }

    fn requires_approval(&self) -> bool {
        true // External HTTP requests require approval
    }

    fn validate_input(&self, input: &Value) -> Result<()> {
        let input: HttpInput = serde_json::from_value(input.clone())
            .map_err(|e| ToolError::InvalidInput(format!("Invalid input: {}", e)))?;

        // Validate URL
        if !input.url.starts_with("http://") && !input.url.starts_with("https://") {
            return Err(ToolError::InvalidInput(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        // Validate timeout
        if input.timeout_secs == 0 || input.timeout_secs > 120 {
            return Err(ToolError::InvalidInput(
                "Timeout must be between 1 and 120 seconds".to_string(),
            ));
        }

        // Validate method
        parse_method(&input.method)?;

        // Validate body is only used with appropriate methods
        if input.body.is_some() {
            let method = parse_method(&input.method)?;
            if !matches!(method, Method::POST | Method::PUT | Method::PATCH) {
                return Err(ToolError::InvalidInput(
                    "Body can only be used with POST, PUT, or PATCH methods".to_string(),
                ));
            }
        }

        Ok(())
    }

    async fn execute(&self, input: Value, _context: &ToolExecutionContext) -> Result<ToolResult> {
        let input: HttpInput = serde_json::from_value(input)?;

        let method = parse_method(&input.method)?;

        // SSRF guard: reject an IP-literal host in a blocked range up
        // front (it never goes through DNS, so the resolver below would
        // never see it); the resolver then covers domain names, including
        // ones that only resolve to an internal address (DNS rebinding);
        // checked_redirect_policy covers a redirect to a blocked address,
        // which check_url_not_blocked alone would never see since it only
        // runs once, before the request.
        if let Err(reason) = super::ssrf_guard::check_url_not_blocked(&input.url) {
            return Ok(ToolResult::error(reason));
        }

        // Build client with timeout
        let client = super::ssrf_guard::guard(
            Client::builder()
                .timeout(StdDuration::from_secs(input.timeout_secs))
                .redirect(if input.follow_redirects {
                    super::ssrf_guard::checked_redirect_policy(10)
                } else {
                    reqwest::redirect::Policy::none()
                }),
        )
        .build()
        .map_err(|e| ToolError::Execution(format!("Failed to build HTTP client: {}", e)))?;

        // Build request
        let mut request = client.request(method.clone(), &input.url);

        // Add headers
        if !input.headers.is_empty() {
            let mut header_map = HeaderMap::new();
            for (key, value) in &input.headers {
                let header_name: reqwest::header::HeaderName = key.parse().map_err(|e| {
                    ToolError::InvalidInput(format!("Invalid header name '{}': {}", key, e))
                })?;
                let header_value: reqwest::header::HeaderValue = value.parse().map_err(|e| {
                    ToolError::InvalidInput(format!("Invalid header value for '{}': {}", key, e))
                })?;
                header_map.insert(header_name, header_value);
            }
            request = request.headers(header_map);
        }

        // Add query parameters
        if !input.query.is_empty() {
            request = request.query(&input.query);
        }

        // Add body if present
        if let Some(body) = input.body {
            request = request.json(&body);
        }

        // Execute request
        let response = request.send().await.map_err(|e| {
            if e.is_timeout() {
                ToolError::Timeout(input.timeout_secs)
            } else if e.is_connect() {
                ToolError::Execution(format!("Connection failed: {}", e))
            } else {
                ToolError::Execution(format!("Request failed: {}", e))
            }
        })?;

        // Extract response details
        let status = response.status();
        let status_code = status.as_u16();
        let is_success = status.is_success();

        // Extract response headers
        let mut response_headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                response_headers.insert(key.to_string(), value_str.to_string());
            }
        }

        // Get response body
        let body_text = response
            .text()
            .await
            .map_err(|e| ToolError::Execution(format!("Failed to read response body: {}", e)))?;

        // Try to parse as JSON, fallback to text
        let body_json: Option<Value> = serde_json::from_str(&body_text).ok();

        // Build output
        let mut output = format!(
            "HTTP {} {}\nStatus: {} {}\n\n",
            input.method.to_uppercase(),
            input.url,
            status_code,
            status.canonical_reason().unwrap_or("Unknown")
        );

        // Add response headers (limit to important ones)
        let important_headers = [
            "content-type",
            "content-length",
            "server",
            "date",
            "location",
            "cache-control",
        ];
        let mut has_headers = false;
        for header in important_headers {
            if let Some(value) = response_headers.get(header) {
                if !has_headers {
                    output.push_str("Headers:\n");
                    has_headers = true;
                }
                output.push_str(&format!("  {}: {}\n", header, value));
            }
        }
        if has_headers {
            output.push('\n');
        }

        // Add response body
        output.push_str("Response Body:\n");
        if let Some(json) = body_json {
            output.push_str(&serde_json::to_string_pretty(&json).unwrap_or(body_text.clone()));
        } else if body_text.is_empty() {
            output.push_str("(empty)");
        } else {
            // Truncate very long text responses
            if body_text.len() > 10000 {
                output.push_str(&format!(
                    "{}... (truncated, {} bytes total)",
                    crate::utils::truncate_at_char_boundary(&body_text, 10000),
                    body_text.len()
                ));
            } else {
                output.push_str(&body_text);
            }
        }

        let mut tool_result = if is_success {
            ToolResult::success(output)
        } else {
            ToolResult::error(output)
        };

        tool_result
            .metadata
            .insert("status_code".to_string(), status_code.to_string());
        tool_result
            .metadata
            .insert("method".to_string(), input.method.to_uppercase());
        tool_result.metadata.insert("url".to_string(), input.url);

        Ok(tool_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regression: no SSRF guard existed - a model could request the
    /// cloud-metadata endpoint (or any other internal service) directly by
    /// IP literal.
    #[tokio::test]
    async fn execute_denies_cloud_metadata_endpoint() {
        let tool = HttpClientTool;
        let context = ToolExecutionContext::new(uuid::Uuid::new_v4()).with_auto_approve(true);
        let input = serde_json::json!({
            "method": "GET",
            "url": "http://169.254.169.254/latest/meta-data/"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success, "must deny the cloud metadata endpoint");
    }

    #[tokio::test]
    async fn execute_denies_loopback_address() {
        let tool = HttpClientTool;
        let context = ToolExecutionContext::new(uuid::Uuid::new_v4()).with_auto_approve(true);
        let input = serde_json::json!({
            "method": "GET",
            "url": "http://127.0.0.1:6379/"
        });

        let result = tool.execute(input, &context).await.unwrap();
        assert!(!result.success, "must deny loopback addresses");
    }
}

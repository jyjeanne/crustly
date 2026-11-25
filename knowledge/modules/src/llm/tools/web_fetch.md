---
type: Rust Module
title: web_fetch
resource: src/llm/tools/web_fetch.rs#L1-L312
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-error-result-toolerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/once-cell-sync-lazy
    resolved_by: tree-sitter
    confidence: exact
  - target: external/regex-regex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [WebFetchTool](../../../../classes/src/llm/tools/web_fetch/WebFetchTool.md)
- [WebFetchInput](../../../../classes/src/llm/tools/web_fetch/WebFetchInput.md)
- [default_timeout](../../../../functions/src/llm/tools/web_fetch/default_timeout.md)
- [default_max_bytes](../../../../functions/src/llm/tools/web_fetch/default_max_bytes.md)
- [default_true](../../../../functions/src/llm/tools/web_fetch/default_true.md)
- [html_to_text](../../../../functions/src/llm/tools/web_fetch/html_to_text.md)
- [name](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/web_fetch/WebFetchTool/tool/execute.md)
- [test_html_to_text_strips_tags](../../../../functions/src/llm/tools/web_fetch/test_html_to_text_strips_tags.md)
- [test_html_to_text_strips_script](../../../../functions/src/llm/tools/web_fetch/test_html_to_text_strips_script.md)
- [test_html_to_text_decodes_entities](../../../../functions/src/llm/tools/web_fetch/test_html_to_text_decodes_entities.md)
- [test_validate_input_rejects_non_http](../../../../functions/src/llm/tools/web_fetch/test_validate_input_rejects_non_http.md)
- [test_validate_input_accepts_https](../../../../functions/src/llm/tools/web_fetch/test_validate_input_accepts_https.md)
- [execute_denies_cloud_metadata_endpoint](../../../../functions/src/llm/tools/web_fetch/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../functions/src/llm/tools/web_fetch/execute_denies_loopback_address.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `once_cell::sync::Lazy`
- `regex::Regex`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
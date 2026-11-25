---
type: Rust Module
title: http
resource: src/llm/tools/http.rs#L1-L362
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
  - target: external/reqwest-header-headermap-client-method
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration-as-stdduration
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

- [HttpClientTool](../../../../classes/src/llm/tools/http/HttpClientTool.md)
- [HttpInput](../../../../classes/src/llm/tools/http/HttpInput.md)
- [default_timeout](../../../../functions/src/llm/tools/http/default_timeout.md)
- [default_true](../../../../functions/src/llm/tools/http/default_true.md)
- [parse_method](../../../../functions/src/llm/tools/http/parse_method.md)
- [name](../../../../functions/src/llm/tools/http/HttpClientTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/http/HttpClientTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/http/HttpClientTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/http/HttpClientTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/http/HttpClientTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/http/HttpClientTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [execute_denies_cloud_metadata_endpoint](../../../../functions/src/llm/tools/http/execute_denies_cloud_metadata_endpoint.md)
- [execute_denies_loopback_address](../../../../functions/src/llm/tools/http/execute_denies_loopback_address.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `reqwest::{header::HeaderMap, Client, Method}`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::collections::HashMap`
- `std::time::Duration as StdDuration`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
---
type: Rust Module
title: http
resource: src/llm/tools/http.rs#L1-L362
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/reqwest-header-headermap-client-method
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-collections-hashmap
  - external/std-time-duration-as-stdduration
  - external/super
  member_of:
  - packages/crustly
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
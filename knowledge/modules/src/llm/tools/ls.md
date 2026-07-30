---
type: Rust Module
title: ls
resource: src/llm/tools/ls.rs#L1-L259
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-path-path-pathbuf
  - external/tokio-fs
  member_of:
  - packages/crustly
---

# Contains

- [LsTool](../../../../classes/src/llm/tools/ls/LsTool.md)
- [LsInput](../../../../classes/src/llm/tools/ls/LsInput.md)
- [name](../../../../functions/src/llm/tools/ls/LsTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/ls/LsTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/ls/LsTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/ls/LsTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/ls/LsTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/ls/LsTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/ls/LsTool/tool/execute.md)
- [list_directory](../../../../functions/src/llm/tools/ls/LsTool/list_directory.md)
- [list_recursive](../../../../functions/src/llm/tools/ls/LsTool/list_recursive.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `tokio::fs`

# Member of

- [crustly](../../../../packages/crustly.md)
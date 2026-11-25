---
type: Rust Module
title: code_exec
resource: src/llm/tools/code_exec.rs#L1-L266
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
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-time-timeout-duration
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [CodeExecTool](../../../../classes/src/llm/tools/code_exec/CodeExecTool.md)
- [CodeExecInput](../../../../classes/src/llm/tools/code_exec/CodeExecInput.md)
- [default_timeout](../../../../functions/src/llm/tools/code_exec/default_timeout.md)
- [name](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/execute.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `tokio::fs`
- `tokio::process::Command`
- `tokio::time::{timeout, Duration}`

# Member of

- [crustly](../../../../packages/crustly.md)
---
type: Rust Module
title: context
resource: src/llm/tools/context.rs#L1-L414
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
  - target: external/chrono-datetime-utc
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
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-fs
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ContextTool](../../../../classes/src/llm/tools/context/ContextTool.md)
- [ContextEntry](../../../../classes/src/llm/tools/context/ContextEntry.md)
- [ContextStore](../../../../classes/src/llm/tools/context/ContextStore.md)
- [new](../../../../functions/src/llm/tools/context/ContextStore/new.md)
- [load](../../../../functions/src/llm/tools/context/ContextStore/load.md)
- [save](../../../../functions/src/llm/tools/context/ContextStore/save.md)
- [ContextOperation](../../../../classes/src/llm/tools/context/ContextOperation.md)
- [ContextInput](../../../../classes/src/llm/tools/context/ContextInput.md)
- [get_store_path](../../../../functions/src/llm/tools/context/get_store_path.md)
- [name](../../../../functions/src/llm/tools/context/ContextTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/context/ContextTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/context/ContextTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/context/ContextTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/context/ContextTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/context/ContextTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `tokio::fs`

# Member of

- [crustly](../../../../packages/crustly.md)
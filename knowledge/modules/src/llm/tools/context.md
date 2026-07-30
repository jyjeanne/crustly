---
type: Rust Module
title: context
resource: src/llm/tools/context.rs#L1-L414
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/chrono-datetime-utc
  - external/serde-deserialize-serialize
  - external/serde-json-value
  - external/std-collections-hashmap
  - external/std-path-path-pathbuf
  - external/tokio-fs
  member_of:
  - packages/crustly
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
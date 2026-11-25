---
type: Rust Module
title: task
resource: src/llm/tools/task.rs#L1-L714
generated:
  by: okf-rs/0.7.0
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
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-io-asyncwriteext
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [TaskTool](../../../../classes/src/llm/tools/task/TaskTool.md)
- [TaskStatus](../../../../classes/src/llm/tools/task/TaskStatus.md)
- [TaskPriority](../../../../classes/src/llm/tools/task/TaskPriority.md)
- [Task](../../../../classes/src/llm/tools/task/Task.md)
- [TaskStore](../../../../classes/src/llm/tools/task/TaskStore.md)
- [short_id](../../../../functions/src/llm/tools/task/short_id.md)
- [FileLock](../../../../classes/src/llm/tools/task/FileLock.md)
- [acquire](../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [release](../../../../functions/src/llm/tools/task/FileLock/release.md)
- [drop](../../../../functions/src/llm/tools/task/FileLock/drop/drop.md)
- [new](../../../../functions/src/llm/tools/task/TaskStore/new.md)
- [load](../../../../functions/src/llm/tools/task/TaskStore/load.md)
- [save](../../../../functions/src/llm/tools/task/TaskStore/save.md)
- [with_lock](../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [TaskOperation](../../../../classes/src/llm/tools/task/TaskOperation.md)
- [TaskInput](../../../../classes/src/llm/tools/task/TaskInput.md)
- [parse_priority](../../../../functions/src/llm/tools/task/parse_priority.md)
- [parse_status](../../../../functions/src/llm/tools/task/parse_status.md)
- [get_store_path](../../../../functions/src/llm/tools/task/get_store_path.md)
- [name](../../../../functions/src/llm/tools/task/TaskTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/task/TaskTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/task/TaskTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/task/TaskTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/task/TaskTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/task/TaskTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/task/TaskTool/tool/execute.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `serde_json::Value`
- `std::collections::HashMap`
- `std::path::{Path, PathBuf}`
- `std::time::Duration`
- `tokio::fs`
- `uuid::Uuid`
- `tokio::io::AsyncWriteExt`

# Member of

- [crustly](../../../../packages/crustly.md)
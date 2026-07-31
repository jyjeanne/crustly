---
type: Rust Module
title: task
resource: src/llm/tools/task.rs#L1-L714
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
  - external/std-time-duration
  - external/tokio-fs
  - external/uuid-uuid
  - external/tokio-io-asyncwriteext
  member_of:
  - packages/crustly
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
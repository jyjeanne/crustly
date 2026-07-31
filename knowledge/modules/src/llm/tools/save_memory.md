---
type: Rust Module
title: save_memory
resource: src/llm/tools/save_memory.rs#L1-L300
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/super-error-result-toolerror
  - external/super-r-trait-tool-toolcapability-toolexecutioncontext-toolresult
  - external/async-trait-async-trait
  - external/serde-deserialize
  - external/serde-json-value
  - external/std-path-path-pathbuf
  - external/tokio-fs
  - external/super
  - external/tempfile-tempdir
  - external/uuid-uuid
  member_of:
  - packages/crustly
---

# Contains

- [SaveMemoryTool](../../../../classes/src/llm/tools/save_memory/SaveMemoryTool.md)
- [SaveMemoryInput](../../../../classes/src/llm/tools/save_memory/SaveMemoryInput.md)
- [memory_path](../../../../functions/src/llm/tools/save_memory/memory_path.md)
- [append_fact](../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [name](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [context](../../../../functions/src/llm/tools/save_memory/context.md)
- [execute_creates_memory_file_with_header_and_fact](../../../../functions/src/llm/tools/save_memory/execute_creates_memory_file_with_header_and_fact.md)
- [execute_appends_to_existing_memory_file](../../../../functions/src/llm/tools/save_memory/execute_appends_to_existing_memory_file.md)
- [execute_does_not_duplicate_an_identical_fact](../../../../functions/src/llm/tools/save_memory/execute_does_not_duplicate_an_identical_fact.md)
- [memory_persists_across_different_sessions_in_the_same_directory](../../../../functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory.md)
- [execute_blocked_in_read_only_mode](../../../../functions/src/llm/tools/save_memory/execute_blocked_in_read_only_mode.md)
- [validate_input_rejects_empty_fact](../../../../functions/src/llm/tools/save_memory/validate_input_rejects_empty_fact.md)
- [append_fact_adds_header_to_a_file_that_lacks_one](../../../../functions/src/llm/tools/save_memory/append_fact_adds_header_to_a_file_that_lacks_one.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)
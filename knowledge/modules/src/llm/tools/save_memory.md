---
type: Rust Module
title: save_memory
resource: src/llm/tools/save_memory.rs#L1-L586
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
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SaveMemoryTool](../../../../classes/src/llm/tools/save_memory/SaveMemoryTool.md)
- [new](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)
- [file_only](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/file_only.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [SaveMemoryInput](../../../../classes/src/llm/tools/save_memory/SaveMemoryInput.md)
- [memory_path](../../../../functions/src/llm/tools/save_memory/memory_path.md)
- [append_fact](../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [name](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/requires_approval.md)
- [requires_approval_for](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/requires_approval_for.md)
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
- [db_context](../../../../functions/src/llm/tools/save_memory/db_context.md)
- [store_with_db](../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [database_save_can_be_global_with_a_category](../../../../functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category.md)
- [database_save_refuses_episodes_bad_expiry_and_secrets](../../../../functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets.md)
- [database_replace_with_an_unknown_id_is_an_error](../../../../functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error.md)
- [only_global_saves_with_a_store_need_approval](../../../../functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `crate::memory::MemoryStore`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `std::path::{Path, PathBuf}`
- `std::sync::Arc`
- `tokio::fs`
- `super::*`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)
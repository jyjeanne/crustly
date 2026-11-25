---
type: Rust Module
title: forget_memory
resource: src/llm/tools/forget_memory.rs#L1-L275
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
  - target: external/crate-memory-model-memory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-recall-short-id
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
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-memoryconfig
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-newmemory-saveoutcome
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

- [ForgetMemoryTool](../../../../classes/src/llm/tools/forget_memory/ForgetMemoryTool.md)
- [new](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [ForgetMemoryInput](../../../../classes/src/llm/tools/forget_memory/ForgetMemoryInput.md)
- [resolve](../../../../functions/src/llm/tools/forget_memory/resolve.md)
- [describe](../../../../functions/src/llm/tools/forget_memory/describe.md)
- [name](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/capabilities.md)
- [validate_input](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/validate_input.md)
- [approval_preview](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/approval_preview.md)
- [execute](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
- [setup](../../../../functions/src/llm/tools/forget_memory/setup.md)
- [save](../../../../functions/src/llm/tools/forget_memory/save.md)
- [context](../../../../functions/src/llm/tools/forget_memory/context.md)
- [the_preview_shows_each_memory_before_anything_is_deleted](../../../../functions/src/llm/tools/forget_memory/the_preview_shows_each_memory_before_anything_is_deleted.md)
- [forgetting_removes_the_memories_and_records_the_actor](../../../../functions/src/llm/tools/forget_memory/forgetting_removes_the_memories_and_records_the_actor.md)
- [one_unknown_id_deletes_nothing](../../../../functions/src/llm/tools/forget_memory/one_unknown_id_deletes_nothing.md)
- [too_many_ids_are_refused](../../../../functions/src/llm/tools/forget_memory/too_many_ids_are_refused.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `crate::memory::model::Memory`
- `crate::memory::recall::short_id`
- `crate::memory::MemoryStore`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `std::sync::Arc`
- `super::*`
- `crate::config::MemoryConfig`
- `crate::db::Database`
- `crate::memory::model::{MemoryCategory, MemoryScope, MemorySource, NewMemory, SaveOutcome}`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)
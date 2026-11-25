---
type: Rust Module
title: search_memory
resource: src/llm/tools/search_memory.rs#L1-L332
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
  - target: external/crate-memory-model-memory-memorycategory-memoryscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-recall-short-id
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-memorystore-searchscope
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
  - target: external/std-str-fromstr
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
  - target: external/crate-memory-model-memorysource-newmemory-saveoutcome
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

- [SearchMemoryTool](../../../../classes/src/llm/tools/search_memory/SearchMemoryTool.md)
- [new](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/new.md)
- [SearchMemoryInput](../../../../classes/src/llm/tools/search_memory/SearchMemoryInput.md)
- [result_line](../../../../functions/src/llm/tools/search_memory/result_line.md)
- [name](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/name.md)
- [description](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/description.md)
- [input_schema](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/input_schema.md)
- [capabilities](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/capabilities.md)
- [requires_approval](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/requires_approval.md)
- [validate_input](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/validate_input.md)
- [execute](../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute.md)
- [setup](../../../../functions/src/llm/tools/search_memory/setup.md)
- [save](../../../../functions/src/llm/tools/search_memory/save.md)
- [finds_project_and_global_memories_by_words](../../../../functions/src/llm/tools/search_memory/finds_project_and_global_memories_by_words.md)
- [scope_and_category_narrow_the_results](../../../../functions/src/llm/tools/search_memory/scope_and_category_narrow_the_results.md)
- [never_returns_another_projects_memories](../../../../functions/src/llm/tools/search_memory/never_returns_another_projects_memories.md)
- [bad_arguments_are_errors_and_search_needs_no_approval](../../../../functions/src/llm/tools/search_memory/bad_arguments_are_errors_and_search_needs_no_approval.md)

# Imports

- `super::error::{Result, ToolError}`
- `super::r#trait::{Tool, ToolCapability, ToolExecutionContext, ToolResult}`
- `crate::memory::model::{Memory, MemoryCategory, MemoryScope}`
- `crate::memory::recall::short_id`
- `crate::memory::{MemoryStore, SearchScope}`
- `async_trait::async_trait`
- `serde::Deserialize`
- `serde_json::Value`
- `std::str::FromStr`
- `std::sync::Arc`
- `super::*`
- `crate::config::MemoryConfig`
- `crate::db::Database`
- `crate::memory::model::{MemorySource, NewMemory, SaveOutcome}`
- `tempfile::TempDir`
- `uuid::Uuid`

# Member of

- [crustly](../../../../packages/crustly.md)
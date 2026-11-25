---
type: Rust Function
title: store_with_path_memory
resource: src/llm/agent/service/run.rs#L677-L712
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/run/an_expired_path_memory_is_not_sent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/a_path_memory_cannot_close_the_notes_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn store_with_path_memory( content: &str, expires_at: Option<chrono::DateTime<chrono::Utc>>, ) -> (Arc<crate::memory::MemoryStore>, tempfile::TempDir)`

# Calls

- [connect_in_memory](../../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../../../functions/src/db/Database/pool.md)
- [default](../../../../../../functions/src/config/MemoryConfig/default/default.md)
- [identify](../../../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../../../../../functions/src/memory/MemoryStore/save.md)
- [add_paths](../../../../../../functions/src/memory/MemoryStore/add_paths.md)

# Called by

- [an_expired_path_memory_is_not_sent](../../../../../../functions/src/llm/agent/service/run/an_expired_path_memory_is_not_sent.md)
- [a_path_memory_cannot_close_the_notes_block](../../../../../../functions/src/llm/agent/service/run/a_path_memory_cannot_close_the_notes_block.md)
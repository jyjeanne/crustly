---
type: Rust Function
title: cmd_memory
resource: src/cli/mod.rs#L1160-L1570
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/embedding_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_status_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/parse_scope
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/resolve_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/events
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/confirm
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/export
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/backup_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/consolidate/MemoryStore/consolidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/pending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/reindex
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/relink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/MemoryCategory/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/model/MemoryStatus/as_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/display_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_memory(config: &crate::config::Config, operation: MemoryCommands) -> Result<()>`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [context](../../../functions/src/services/ServiceManager/context.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [build_memory_store](../../../functions/src/cli/build_memory_store.md)
- [identify](../../../functions/src/memory/project/identify.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [list](../../../functions/src/memory/MemoryStore/list.md)
- [embedding_status](../../../functions/src/memory/MemoryStore/embedding_status.md)
- [memory_status_lines](../../../functions/src/cli/memory_status_lines.md)
- [parse_scope](../../../functions/src/cli/parse_scope.md)
- [resolve_memory](../../../functions/src/cli/resolve_memory.md)
- [events](../../../functions/src/memory/MemoryStore/events.md)
- [search](../../../functions/src/memory/MemoryStore/search.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [save](../../../functions/src/memory/MemoryStore/save.md)
- [add_paths](../../../functions/src/memory/MemoryStore/add_paths.md)
- [edit](../../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../../functions/src/memory/MemoryStore/promote.md)
- [all](../../../functions/src/memory/MemoryStore/all.md)
- [confirm](../../../functions/src/cli/confirm.md)
- [forget](../../../functions/src/memory/MemoryStore/forget.md)
- [export](../../../functions/src/memory/transfer/export.md)
- [write](../../../functions/src/memory/project/write.md)
- [backup_stamp](../../../functions/src/memory/transfer/backup_stamp.md)
- [import_from_notes](../../../functions/src/memory/transfer/import_from_notes.md)
- [import](../../../functions/src/memory/transfer/import.md)
- [projects](../../../functions/src/memory/MemoryStore/projects.md)
- [create_provider](../../../functions/src/llm/provider/factory/create_provider.md)
- [consolidate](../../../functions/src/memory/consolidate/MemoryStore/consolidate.md)
- [pending](../../../functions/src/memory/extraction/MemoryStore/pending.md)
- [reindex](../../../functions/src/memory/MemoryStore/reindex.md)
- [relink](../../../functions/src/memory/MemoryStore/relink.md)
- [memory_line](../../../functions/src/cli/memory_line.md)
- [as_str](../../../functions/src/memory/model/MemoryCategory/as_str.md)
- [as_str](../../../functions/src/memory/model/MemoryStatus/as_str.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [display_path](../../../functions/src/cli/display_path.md)

# Called by

- [run](../../../functions/src/cli/run.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
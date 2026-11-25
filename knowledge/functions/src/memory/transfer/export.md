---
type: Rust Function
title: export
resource: src/memory/transfer.rs#L66-L112
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_scope_limits_what_is_written
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/an_unknown_export_version_is_refused
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn export(store: &MemoryStore, scope: ExportScope) -> Result<ExportFile>`

# Calls

- [projects](../../../../functions/src/memory/MemoryStore/projects.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [export_scope_limits_what_is_written](../../../../functions/tests/memory_transfer_test/export_scope_limits_what_is_written.md)
- [an_unknown_export_version_is_refused](../../../../functions/tests/memory_transfer_test/an_unknown_export_version_is_refused.md)
---
type: Rust Function
title: project_at
resource: tests/memory_transfer_test.rs#L19-L26
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_scope_limits_what_is_written
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn project_at(root: &std::path::Path) -> ProjectIdentity`

# Calls

- [id_for_root](../../../functions/src/memory/project/id_for_root.md)

# Called by

- [export_then_import_recreates_the_memories_in_a_new_database](../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [export_scope_limits_what_is_written](../../../functions/tests/memory_transfer_test/export_scope_limits_what_is_written.md)
---
type: Rust Method
title: find_by_hash
resource: src/db/repository/memory.rs#L129-L141
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/scope_columns
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/promote
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/find_by_hash_matches_only_the_same_scope
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn find_by_hash(&self, scope: &MemoryScope, hash: &str) -> Result<Option<Memory>>`

# Calls

- [scope_columns](../../../../../../functions/src/db/repository/memory/scope_columns.md)

# Called by

- [save_with](../../../../../../functions/src/memory/MemoryStore/save_with.md)
- [edit](../../../../../../functions/src/memory/MemoryStore/edit.md)
- [promote](../../../../../../functions/src/memory/MemoryStore/promote.md)
- [find_by_hash_matches_only_the_same_scope](../../../../../../functions/tests/memory_repository_test/find_by_hash_matches_only_the_same_scope.md)
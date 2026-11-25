---
type: Rust Function
title: find_by_hash_matches_only_the_same_scope
resource: tests/memory_repository_test.rs#L79-L100
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/new_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/project_scope
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/find_by_hash
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn find_by_hash_matches_only_the_same_scope()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [project_scope](../../../functions/tests/memory_repository_test/project_scope.md)
- [find_by_hash](../../../functions/src/db/repository/memory/MemoryRepository/find_by_hash.md)
---
type: Rust Function
title: fts_search_never_crosses_projects
resource: tests/memory_repository_test.rs#L206-L215
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/db/repository/memory/MemoryRepository/fts_search
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fixed_now
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn fts_search_never_crosses_projects()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [project_scope](../../../functions/tests/memory_repository_test/project_scope.md)
- [fts_search](../../../functions/src/db/repository/memory/MemoryRepository/fts_search.md)
- [fixed_now](../../../functions/tests/memory_repository_test/fixed_now.md)
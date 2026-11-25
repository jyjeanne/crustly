---
type: Rust Function
title: relink_moves_memories_to_the_new_project
resource: tests/memory_repository_test.rs#L288-L305
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
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fixed_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/relink_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn relink_moves_memories_to_the_new_project()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [project_scope](../../../functions/tests/memory_repository_test/project_scope.md)
- [upsert_project](../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [fixed_now](../../../functions/tests/memory_repository_test/fixed_now.md)
- [relink_project](../../../functions/src/db/repository/memory/MemoryRepository/relink_project.md)
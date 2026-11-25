---
type: Rust Function
title: saved_memory_reads_back_unchanged
resource: tests/memory_repository_test.rs#L61-L76
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
---

# Signature

`async fn saved_memory_reads_back_unchanged()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [project_scope](../../../functions/tests/memory_repository_test/project_scope.md)
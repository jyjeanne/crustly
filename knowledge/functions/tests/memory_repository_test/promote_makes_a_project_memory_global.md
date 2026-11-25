---
type: Rust Function
title: promote_makes_a_project_memory_global
resource: tests/memory_repository_test.rs#L271-L285
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
  - target: functions/tests/memory_repository_test/fixed_now
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn promote_makes_a_project_memory_global()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [project_scope](../../../functions/tests/memory_repository_test/project_scope.md)
- [fixed_now](../../../functions/tests/memory_repository_test/fixed_now.md)
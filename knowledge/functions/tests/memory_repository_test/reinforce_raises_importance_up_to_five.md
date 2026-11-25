---
type: Rust Function
title: reinforce_raises_importance_up_to_five
resource: tests/memory_repository_test.rs#L103-L114
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
  - target: functions/src/db/repository/memory/MemoryRepository/reinforce
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fixed_now
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn reinforce_raises_importance_up_to_five()`

# Calls

- [repo_with_projects](../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [new_memory](../../../functions/tests/memory_repository_test/new_memory.md)
- [reinforce](../../../functions/src/db/repository/memory/MemoryRepository/reinforce.md)
- [fixed_now](../../../functions/tests/memory_repository_test/fixed_now.md)
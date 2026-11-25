---
type: Rust Function
title: store_and_project
resource: tests/memory_paths_test.rs#L11-L23
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_paths_test/paths_must_be_relative_and_few
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn store_and_project(dir: &Path) -> (Database, MemoryStore, ProjectIdentity)`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)

# Called by

- [paths_must_be_relative_and_few](../../../functions/tests/memory_paths_test/paths_must_be_relative_and_few.md)
- [a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget](../../../functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget.md)
- [a_path_memory_is_shown_once_per_session](../../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)
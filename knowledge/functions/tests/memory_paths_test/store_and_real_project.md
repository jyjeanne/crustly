---
type: Rust Function
title: store_and_real_project
resource: tests/memory_paths_test.rs#L170-L192
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn store_and_real_project(dir: &Path) -> (Database, MemoryStore, ProjectIdentity)`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [write](../../../functions/src/memory/project/write.md)
- [id_for_root](../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)

# Called by

- [tool_paths_match_when_absolute_or_relative_to_a_subdirectory](../../../functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory.md)
- [an_absolute_path_in_the_message_brings_its_memory_in](../../../functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in.md)
---
type: Rust Function
title: setup
resource: src/llm/tools/search_memory.rs#L164-L179
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
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
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn setup(dir: &TempDir) -> (Database, SearchMemoryTool, ToolExecutionContext)`

# Calls

- [connect_in_memory](../../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../../../functions/src/db/Database/pool.md)
- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [identify](../../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../../functions/src/memory/MemoryStore/register_project.md)
- [new](../../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/new.md)
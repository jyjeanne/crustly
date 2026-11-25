---
type: Rust Function
title: forget_is_refused_in_plan_mode_and_deletes_nothing
resource: tests/memory_review_fixes_test.rs#L145-L166
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
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn forget_is_refused_in_plan_mode_and_deletes_nothing()`

# Calls

- [connect_in_memory](../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [new](../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [upsert_project](../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [insert](../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [new](../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [new](../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [execute](../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
- [get](../../../functions/src/db/repository/memory/MemoryRepository/get.md)
---
type: Rust Function
title: database_save_can_be_global_with_a_category
resource: src/llm/tools/save_memory.rs#L519-L532
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/save_memory/store_with_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/db_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn database_save_can_be_global_with_a_category()`

# Calls

- [store_with_db](../../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [new](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [db_context](../../../../../functions/src/llm/tools/save_memory/db_context.md)
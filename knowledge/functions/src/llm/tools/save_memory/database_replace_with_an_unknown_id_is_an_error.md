---
type: Rust Function
title: database_replace_with_an_unknown_id_is_an_error
resource: src/llm/tools/save_memory.rs#L553-L569
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`async fn database_replace_with_an_unknown_id_is_an_error()`

# Calls

- [store_with_db](../../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [new](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [db_context](../../../../../functions/src/llm/tools/save_memory/db_context.md)
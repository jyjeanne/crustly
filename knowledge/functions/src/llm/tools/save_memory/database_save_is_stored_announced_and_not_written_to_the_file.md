---
type: Rust Function
title: database_save_is_stored_announced_and_not_written_to_the_file
resource: src/llm/tools/save_memory.rs#L501-L516
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
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn database_save_is_stored_announced_and_not_written_to_the_file()`

# Calls

- [store_with_db](../../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [new](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/new.md)
- [execute](../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)
- [db_context](../../../../../functions/src/llm/tools/save_memory/db_context.md)
- [exists](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [take_notices](../../../../../functions/src/memory/MemoryStore/take_notices.md)
---
type: Rust Function
title: forgetting_removes_the_memories_and_records_the_actor
resource: src/llm/tools/forget_memory.rs#L223-L245
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn forgetting_removes_the_memories_and_records_the_actor()`

# Calls

- [new](../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [execute](../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
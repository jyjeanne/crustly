---
type: Rust Function
title: one_unknown_id_deletes_nothing
resource: src/llm/tools/forget_memory.rs#L248-L264
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn one_unknown_id_deletes_nothing()`

# Calls

- [new](../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [execute](../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
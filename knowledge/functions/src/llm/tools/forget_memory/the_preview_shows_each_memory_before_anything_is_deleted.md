---
type: Rust Function
title: the_preview_shows_each_memory_before_anything_is_deleted
resource: src/llm/tools/forget_memory.rs#L207-L220
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
  - target: functions/src/llm/tools/trait/Tool/requires_approval_for
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn the_preview_shows_each_memory_before_anything_is_deleted()`

# Calls

- [new](../../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/new.md)
- [requires_approval_for](../../../../../functions/src/llm/tools/trait/Tool/requires_approval_for.md)
---
type: Rust Function
title: find_after_returns_an_absolute_offset_not_a_relative_one
resource: src/llm/provider/qwen.rs#L1897-L1902
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_after_returns_an_absolute_offset_not_a_relative_one()`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [find_after](../../../../../functions/src/llm/provider/qwen/find_after.md)
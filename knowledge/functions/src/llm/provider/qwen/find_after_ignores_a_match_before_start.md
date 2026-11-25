---
type: Rust Function
title: find_after_ignores_a_match_before_start
resource: src/llm/provider/qwen.rs#L1880-L1887
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn find_after_ignores_a_match_before_start()`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [find_after](../../../../../functions/src/llm/provider/qwen/find_after.md)
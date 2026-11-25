---
type: Rust Function
title: find_after_returns_none_when_nothing_matches_after_start
resource: src/llm/provider/qwen.rs#L1890-L1894
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

`fn find_after_returns_none_when_nothing_matches_after_start()`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [find_after](../../../../../functions/src/llm/provider/qwen/find_after.md)
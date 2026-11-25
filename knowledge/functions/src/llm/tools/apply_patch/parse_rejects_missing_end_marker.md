---
type: Rust Function
title: parse_rejects_missing_end_marker
resource: src/llm/tools/apply_patch.rs#L553-L556
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/apply_patch/parse_patch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_rejects_missing_end_marker()`

# Calls

- [parse_patch](../../../../../functions/src/llm/tools/apply_patch/parse_patch.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
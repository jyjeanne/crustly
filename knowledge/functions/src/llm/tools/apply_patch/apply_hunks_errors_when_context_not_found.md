---
type: Rust Function
title: apply_hunks_errors_when_context_not_found
resource: src/llm/tools/apply_patch.rs#L638-L645
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/apply_patch/apply_hunks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn apply_hunks_errors_when_context_not_found()`

# Calls

- [apply_hunks](../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
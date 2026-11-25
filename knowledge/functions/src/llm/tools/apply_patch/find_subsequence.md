---
type: Rust Function
title: find_subsequence
resource: src/llm/tools/apply_patch.rs#L220-L229
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/apply_patch/apply_hunks
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_subsequence(haystack: &[String], needle: &[&str], start: usize) -> Option<usize>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [apply_hunks](../../../../../functions/src/llm/tools/apply_patch/apply_hunks.md)
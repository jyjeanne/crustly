---
type: Rust Function
title: first_line
resource: src/llm/agent/project_memory.rs#L204-L215
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_line(text: &str, max_chars: usize) -> String`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
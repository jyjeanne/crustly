---
type: Rust Function
title: resolve
resource: src/llm/tools/aliases.rs#L83-L88
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve(name: &str) -> Option<&'static str>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
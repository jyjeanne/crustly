---
type: Rust Function
title: resolve
resource: src/llm/tools/forget_memory.rs#L39-L67
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn resolve(store: &MemoryStore, ids: &[String]) -> std::result::Result<Vec<Memory>, String>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
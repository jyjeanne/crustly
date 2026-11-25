---
type: Rust Function
title: save
resource: src/llm/tools/forget_memory.rs#L181-L199
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save(store: &MemoryStore, content: &str) -> Uuid`

# Calls

- [save](../../../../../functions/src/memory/MemoryStore/save.md)
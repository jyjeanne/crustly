---
type: Rust Function
title: save
resource: src/llm/tools/search_memory.rs#L181-L202
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save( tool: &SearchMemoryTool, scope: MemoryScope, content: &str, category: MemoryCategory, )`

# Calls

- [save](../../../../../functions/src/memory/MemoryStore/save.md)
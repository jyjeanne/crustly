---
type: Rust Function
title: finds_project_and_global_memories_by_words
resource: src/llm/tools/search_memory.rs#L205-L239
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn finds_project_and_global_memories_by_words()`

# Calls

- [identify](../../../../../functions/src/memory/project/identify.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
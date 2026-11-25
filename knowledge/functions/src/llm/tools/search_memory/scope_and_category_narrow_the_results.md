---
type: Rust Function
title: scope_and_category_narrow_the_results
resource: src/llm/tools/search_memory.rs#L242-L282
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

`async fn scope_and_category_narrow_the_results()`

# Calls

- [identify](../../../../../functions/src/memory/project/identify.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
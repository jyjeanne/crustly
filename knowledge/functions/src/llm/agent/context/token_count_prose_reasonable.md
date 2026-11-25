---
type: Rust Function
title: token_count_prose_reasonable
resource: src/llm/agent/context.rs#L386-L395
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn token_count_prose_reasonable()`

# Calls

- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
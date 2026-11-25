---
type: Rust Function
title: mentions_context_overflow
resource: src/llm/provider/error.rs#L108-L121
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/error/ProviderError/is_context_overflow
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mentions_context_overflow(text: &str) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [is_context_overflow](../../../../../functions/src/llm/provider/error/ProviderError/is_context_overflow.md)
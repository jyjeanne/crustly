---
type: Rust Function
title: disabled_anthropic_fallback_fails_with_a_clear_message
resource: src/llm/provider/factory.rs#L1108-L1131
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn disabled_anthropic_fallback_fails_with_a_clear_message()`

# Calls

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
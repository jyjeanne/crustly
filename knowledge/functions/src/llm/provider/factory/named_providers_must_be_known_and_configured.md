---
type: Rust Function
title: named_providers_must_be_known_and_configured
resource: src/llm/provider/factory.rs#L738-L751
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn named_providers_must_be_known_and_configured()`

# Calls

- [default](../../../../../functions/src/config/Config/default/default.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
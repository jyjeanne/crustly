---
type: Rust Function
title: test_create_provider_no_credentials
resource: src/llm/provider/factory.rs#L1243-L1259
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn test_create_provider_no_credentials()`

# Calls

- [create_provider](../../../../../functions/src/llm/provider/factory/create_provider.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
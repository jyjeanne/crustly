---
type: Rust Function
title: rejects_provably_wrong_arguments
resource: src/llm/tools/schema_check.rs#L173-L182
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/schema_check/check_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/schema_check/schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rejects_provably_wrong_arguments()`

# Calls

- [check_arguments](../../../../../functions/src/llm/tools/schema_check/check_arguments.md)
- [schema](../../../../../functions/src/llm/tools/schema_check/schema.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
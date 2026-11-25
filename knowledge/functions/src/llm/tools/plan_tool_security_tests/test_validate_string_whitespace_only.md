---
type: Rust Function
title: test_validate_string_whitespace_only
resource: src/llm/tools/plan_tool_security_tests.rs#L112-L116
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/plan_tool/validate_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_validate_string_whitespace_only()`

# Calls

- [validate_string](../../../../../functions/src/llm/tools/plan_tool/validate_string.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
---
type: Rust Function
title: test_validate_filename_pattern
resource: src/llm/tools/plan_tool_security_tests.rs#L55-L68
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/plan_tool/validate_plan_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_validate_filename_pattern()`

# Calls

- [validate_plan_file_path](../../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
---
type: Rust Function
title: test_validate_path_outside_working_directory
resource: src/llm/tools/plan_tool_security_tests.rs#L24-L38
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn test_validate_path_outside_working_directory()`

# Calls

- [validate_plan_file_path](../../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
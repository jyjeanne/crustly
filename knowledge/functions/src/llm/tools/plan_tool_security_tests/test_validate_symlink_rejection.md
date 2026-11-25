---
type: Rust Function
title: test_validate_symlink_rejection
resource: src/llm/tools/plan_tool_security_tests.rs#L85-L102
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/plan_tool/validate_plan_file_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_validate_symlink_rejection()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [validate_plan_file_path](../../../../../functions/src/llm/tools/plan_tool/validate_plan_file_path.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
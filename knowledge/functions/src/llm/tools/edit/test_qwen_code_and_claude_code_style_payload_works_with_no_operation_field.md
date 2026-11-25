---
type: Rust Function
title: test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field
resource: src/llm/tools/edit.rs#L471-L489
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
  - target: functions/src/llm/tools/edit/EditTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_qwen_code_and_claude_code_style_payload_works_with_no_operation_field()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)
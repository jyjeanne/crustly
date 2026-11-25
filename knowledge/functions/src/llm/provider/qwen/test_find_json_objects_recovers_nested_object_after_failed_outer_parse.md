---
type: Rust Function
title: test_find_json_objects_recovers_nested_object_after_failed_outer_parse
resource: src/llm/provider/qwen.rs#L2725-L2736
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/find_json_objects
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_find_json_objects_recovers_nested_object_after_failed_outer_parse()`

# Calls

- [find_json_objects](../../../../../functions/src/llm/provider/qwen/QwenProvider/find_json_objects.md)
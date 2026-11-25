---
type: Rust Method
title: find_json_objects
resource: src/llm/provider/qwen.rs#L350-L405
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_find_json_objects_recovers_nested_object_after_failed_outer_parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_json_objects(text: &str) -> Vec<(usize, usize, serde_json::Value)>`

# Called by

- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [test_find_json_objects_recovers_nested_object_after_failed_outer_parse](../../../../../../functions/src/llm/provider/qwen/test_find_json_objects_recovers_nested_object_after_failed_outer_parse.md)
---
type: Rust Method
title: parse_fallback_tool_calls
resource: src/llm/provider/qwen.rs#L439-L478
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/find_json_objects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/generate_call_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/expand_span_over_adjacent_fences
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/push_fallback_or_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_rejects_unregistered_tool_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_fallback_ignores_unrelated_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_fallback_tool_calls( &self, text: &str, known_tools: &[String], ) -> (Vec<(String, String, serde_json::Value)>, String)`

# Calls

- [find_json_objects](../../../../../../functions/src/llm/provider/qwen/QwenProvider/find_json_objects.md)
- [generate_call_id](../../../../../../functions/src/llm/provider/qwen/QwenProvider/generate_call_id.md)
- [expand_span_over_adjacent_fences](../../../../../../functions/src/llm/provider/qwen/QwenProvider/expand_span_over_adjacent_fences.md)

# Called by

- [push_fallback_or_text](../../../../../../functions/src/llm/provider/qwen/QwenProvider/push_fallback_or_text.md)
- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_fallback_parses_bare_json_tool_call](../../../../../../functions/src/llm/provider/qwen/test_fallback_parses_bare_json_tool_call.md)
- [test_fallback_rejects_unregistered_tool_name](../../../../../../functions/src/llm/provider/qwen/test_fallback_rejects_unregistered_tool_name.md)
- [test_fallback_parses_fenced_json_tool_call](../../../../../../functions/src/llm/provider/qwen/test_fallback_parses_fenced_json_tool_call.md)
- [test_fallback_does_not_corrupt_unrelated_fenced_code_block](../../../../../../functions/src/llm/provider/qwen/test_fallback_does_not_corrupt_unrelated_fenced_code_block.md)
- [test_fallback_ignores_unrelated_json](../../../../../../functions/src/llm/provider/qwen/test_fallback_ignores_unrelated_json.md)
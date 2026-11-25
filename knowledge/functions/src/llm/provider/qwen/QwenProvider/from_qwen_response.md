---
type: Rust Method
title: from_qwen_response
resource: src/llm/provider/qwen.rs#L988-L1226
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/extract_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/find_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/push_fallback_or_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/clean_incomplete_markers
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/stream_events_from_buffered_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_uses_fallback_when_no_hermes_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from_qwen_response(&self, response: QwenResponse, known_tools: &[String]) -> LLMResponse`

# Calls

- [next](../../../../../../functions/src/tui/events/EventHandler/next.md)
- [extract_thinking](../../../../../../functions/src/llm/provider/qwen/QwenProvider/extract_thinking.md)
- [parse_hermes_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_hermes_tool_calls.md)
- [find](../../../../../../functions/src/memory/MemoryStore/find.md)
- [find_after](../../../../../../functions/src/llm/provider/qwen/find_after.md)
- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [push_fallback_or_text](../../../../../../functions/src/llm/provider/qwen/QwenProvider/push_fallback_or_text.md)
- [parse_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_native_qwen_tool_calls.md)
- [clean_incomplete_markers](../../../../../../functions/src/llm/provider/qwen/QwenProvider/clean_incomplete_markers.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [stream_events_from_buffered_content](../../../../../../functions/src/llm/provider/qwen/stream_events_from_buffered_content.md)
- [test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_drops_truncated_trailing_hermes_tag_from_display.md)
- [test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_stray_closing_tag_before_real_call_does_not_loop_forever.md)
- [test_from_qwen_response_uses_fallback_when_no_hermes_tags](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_uses_fallback_when_no_hermes_tags.md)
- [test_from_qwen_response_openai_parser_still_detects_fallback_json](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json.md)
- [test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_detects_bare_json_call_mixed_with_hermes_call.md)
---
type: Rust Method
title: with_tool_parser
resource: src/llm/provider/qwen.rs#L169-L172
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/configure_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_tool_parser_configuration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_native_qwen_parser_configuration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_native_qwen_tools_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_native_qwen_result_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_clean_incomplete_markers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_tool_parser(mut self, parser: ToolCallParser) -> Self`

# Called by

- [configure_qwen](../../../../../../functions/src/llm/provider/factory/configure_qwen.md)
- [test_tool_parser_configuration](../../../../../../functions/src/llm/provider/qwen/test_tool_parser_configuration.md)
- [test_native_qwen_parser_configuration](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_parser_configuration.md)
- [test_native_qwen_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing.md)
- [test_multiple_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls.md)
- [test_native_qwen_tools_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tools_format.md)
- [test_native_qwen_result_format](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_result_format.md)
- [test_clean_incomplete_markers](../../../../../../functions/src/llm/provider/qwen/test_clean_incomplete_markers.md)
- [test_from_qwen_response_openai_parser_still_detects_fallback_json](../../../../../../functions/src/llm/provider/qwen/test_from_qwen_response_openai_parser_still_detects_fallback_json.md)
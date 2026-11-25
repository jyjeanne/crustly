---
type: Rust Method
title: parse_native_qwen_tool_calls
resource: src/llm/provider/qwen.rs#L537-L593
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/generate_call_id
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_native_qwen_tool_calls(&self, text: &str) -> Vec<(String, String, serde_json::Value)>`

# Calls

- [skip](../../../../../../functions/src/plan/PlanTask/skip.md)
- [find](../../../../../../functions/src/memory/MemoryStore/find.md)
- [generate_call_id](../../../../../../functions/src/llm/provider/qwen/QwenProvider/generate_call_id.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
- [test_native_qwen_tool_call_parsing](../../../../../../functions/src/llm/provider/qwen/test_native_qwen_tool_call_parsing.md)
- [test_multiple_native_qwen_tool_calls](../../../../../../functions/src/llm/provider/qwen/test_multiple_native_qwen_tool_calls.md)
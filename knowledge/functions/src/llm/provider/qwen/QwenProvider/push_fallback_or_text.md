---
type: Rust Method
title: push_fallback_or_text
resource: src/llm/provider/qwen.rs#L944-L971
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/qwen/QwenProvider/from_qwen_response
---

# Signature

`fn push_fallback_or_text( &self, remaining: String, known_tools: &[String], has_tool_calls: &mut bool, content_blocks: &mut Vec<ContentBlock>, )`

# Calls

- [parse_fallback_tool_calls](../../../../../../functions/src/llm/provider/qwen/QwenProvider/parse_fallback_tool_calls.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [from_qwen_response](../../../../../../functions/src/llm/provider/qwen/QwenProvider/from_qwen_response.md)
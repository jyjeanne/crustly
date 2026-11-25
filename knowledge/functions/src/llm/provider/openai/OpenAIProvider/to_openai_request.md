---
type: Rust Method
title: to_openai_request
resource: src/llm/provider/openai.rs#L188-L356
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/OpenAIProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_openai_request(&self, request: LLMRequest) -> OpenAIRequest`

# Called by

- [complete](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/openai/OpenAIProvider/provider/stream.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them.md)
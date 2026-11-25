---
type: Rust Method
title: to_ollama_request
resource: src/llm/provider/ollama.rs#L282-L421
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/OllamaProvider/overrides_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/invalid_think_value_is_ignored
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_ollama_request(&self, request: LLMRequest) -> ChatMessageRequest`

# Calls

- [overrides_for](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [to_ollama_request_maps_tool_messages](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages.md)
- [to_ollama_request_keeps_text_sent_with_tool_results](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_keeps_text_sent_with_tool_results.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [to_ollama_request_embeds_base64_image](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image.md)
- [invalid_think_value_is_ignored](../../../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
---
type: Rust Method
title: to_ollama_request
resource: src/llm/provider/ollama.rs#L282-L416
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/types/Message/system
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/ollama/OllamaProvider/overrides_for
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/complete
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
  - functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields
  - functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages
  - functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
  - functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking
  - functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
  - functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image
---

# Signature

`fn to_ollama_request(&self, request: LLMRequest) -> ChatMessageRequest`

# Calls

- [system](../../../../../../functions/src/llm/provider/types/Message/system.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [overrides_for](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/overrides_for.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [test_to_ollama_request_maps_common_fields](../../../../../../functions/src/llm/provider/ollama/test_to_ollama_request_maps_common_fields.md)
- [to_ollama_request_maps_tool_messages](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_tool_messages.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [per_model_think_false_is_sent_when_request_has_no_thinking](../../../../../../functions/src/llm/provider/ollama/per_model_think_false_is_sent_when_request_has_no_thinking.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [to_ollama_request_embeds_base64_image](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_embeds_base64_image.md)
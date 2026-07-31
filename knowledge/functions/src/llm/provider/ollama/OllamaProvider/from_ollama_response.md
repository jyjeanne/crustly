---
type: Rust Method
title: from_ollama_response
resource: src/llm/provider/ollama.rs#L420-L508
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/types/extract_think_tags
  - functions/src/llm/provider/ollama/tool_call_from_content
  - functions/src/llm/provider/ollama/perf_metrics_from_final_data
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/complete
  - functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data
  - functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf
  - functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls
  - functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field
  - functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags
---

# Signature

`fn from_ollama_response( &self, response: ChatMessageResponse, offered_tools: &[Tool], ) -> LLMResponse`

# Calls

- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [extract_think_tags](../../../../../../functions/src/llm/provider/types/extract_think_tags.md)
- [tool_call_from_content](../../../../../../functions/src/llm/provider/ollama/tool_call_from_content.md)
- [perf_metrics_from_final_data](../../../../../../functions/src/llm/provider/ollama/perf_metrics_from_final_data.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/complete.md)
- [recovered_tool_call_becomes_a_tool_use_block](../../../../../../functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block.md)
- [fenced_call_in_prose_becomes_a_tool_use_block](../../../../../../functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block.md)
- [from_ollama_response_plain_text_with_final_data](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data.md)
- [from_ollama_response_without_final_data_has_zero_usage_and_no_perf](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf.md)
- [from_ollama_response_extracts_tool_calls](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls.md)
- [from_ollama_response_uses_explicit_thinking_field](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field.md)
- [from_ollama_response_falls_back_to_think_tags](../../../../../../functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags.md)
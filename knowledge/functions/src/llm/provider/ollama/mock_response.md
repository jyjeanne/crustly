---
type: Rust Function
title: mock_response
resource: src/llm/provider/ollama.rs#L1154-L1163
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data
  - functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf
  - functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls
  - functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field
  - functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags
---

# Signature

`fn mock_response(message: ChatMessage, done: bool) -> ChatMessageResponse`

# Called by

- [recovered_tool_call_becomes_a_tool_use_block](../../../../../functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block.md)
- [fenced_call_in_prose_becomes_a_tool_use_block](../../../../../functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block.md)
- [from_ollama_response_plain_text_with_final_data](../../../../../functions/src/llm/provider/ollama/from_ollama_response_plain_text_with_final_data.md)
- [from_ollama_response_without_final_data_has_zero_usage_and_no_perf](../../../../../functions/src/llm/provider/ollama/from_ollama_response_without_final_data_has_zero_usage_and_no_perf.md)
- [from_ollama_response_extracts_tool_calls](../../../../../functions/src/llm/provider/ollama/from_ollama_response_extracts_tool_calls.md)
- [from_ollama_response_uses_explicit_thinking_field](../../../../../functions/src/llm/provider/ollama/from_ollama_response_uses_explicit_thinking_field.md)
- [from_ollama_response_falls_back_to_think_tags](../../../../../functions/src/llm/provider/ollama/from_ollama_response_falls_back_to_think_tags.md)
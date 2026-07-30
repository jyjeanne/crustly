---
type: Rust Function
title: tool_call_from_content
resource: src/llm/provider/ollama.rs#L825-L857
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/parse_tool_call_object
  - functions/src/llm/provider/ollama/fenced_json_blocks
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
  - functions/src/llm/provider/ollama/tool_call_printed_as_content_is_recovered
  - functions/src/llm/provider/ollama/tool_call_in_a_json_fence_is_recovered
  - functions/src/llm/provider/ollama/tool_call_in_a_fence_embedded_in_prose_is_recovered
  - functions/src/llm/provider/ollama/first_of_several_fenced_calls_is_recovered
---

# Signature

`fn tool_call_from_content(content: &str, offered: &[Tool]) -> Option<(String, serde_json::Value)>`

# Calls

- [parse_tool_call_object](../../../../../functions/src/llm/provider/ollama/parse_tool_call_object.md)
- [fenced_json_blocks](../../../../../functions/src/llm/provider/ollama/fenced_json_blocks.md)

# Called by

- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
- [tool_call_printed_as_content_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_printed_as_content_is_recovered.md)
- [tool_call_in_a_json_fence_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_in_a_json_fence_is_recovered.md)
- [tool_call_in_a_fence_embedded_in_prose_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_in_a_fence_embedded_in_prose_is_recovered.md)
- [first_of_several_fenced_calls_is_recovered](../../../../../functions/src/llm/provider/ollama/first_of_several_fenced_calls_is_recovered.md)
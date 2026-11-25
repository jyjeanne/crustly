---
type: Rust Function
title: tool_call_from_content
resource: src/llm/provider/tool_call_recovery.rs#L167-L203
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/tool_call_recovery/parse_tool_call_object
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/fenced_json_blocks
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/run_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaStreamState/finish
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_printed_as_content_is_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_in_a_json_fence_is_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_in_a_fence_embedded_in_prose_is_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/first_of_several_fenced_calls_is_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/fenced_non_tool_json_is_not_recovered
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/prose_is_never_mistaken_for_a_tool_call
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn tool_call_from_content( content: &str, offered: &[Tool], ) -> Option<(String, serde_json::Value)>`

# Calls

- [parse_tool_call_object](../../../../../functions/src/llm/provider/tool_call_recovery/parse_tool_call_object.md)
- [fenced_json_blocks](../../../../../functions/src/llm/provider/tool_call_recovery/fenced_json_blocks.md)

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [from_ollama_response](../../../../../functions/src/llm/provider/ollama/OllamaProvider/from_ollama_response.md)
- [finish](../../../../../functions/src/llm/provider/ollama/OllamaStreamState/finish.md)
- [tool_call_printed_as_content_is_recovered](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_printed_as_content_is_recovered.md)
- [tool_call_in_a_json_fence_is_recovered](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_in_a_json_fence_is_recovered.md)
- [tool_call_in_a_fence_embedded_in_prose_is_recovered](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_in_a_fence_embedded_in_prose_is_recovered.md)
- [first_of_several_fenced_calls_is_recovered](../../../../../functions/src/llm/provider/tool_call_recovery/first_of_several_fenced_calls_is_recovered.md)
- [fenced_non_tool_json_is_not_recovered](../../../../../functions/src/llm/provider/tool_call_recovery/fenced_non_tool_json_is_not_recovered.md)
- [prose_is_never_mistaken_for_a_tool_call](../../../../../functions/src/llm/provider/tool_call_recovery/prose_is_never_mistaken_for_a_tool_call.md)
---
type: Rust Function
title: bash_tool
resource: src/llm/provider/ollama.rs#L1286-L1296
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama/tool_call_printed_as_content_is_recovered
  - functions/src/llm/provider/ollama/tool_call_in_a_json_fence_is_recovered
  - functions/src/llm/provider/ollama/tool_call_in_a_fence_embedded_in_prose_is_recovered
  - functions/src/llm/provider/ollama/first_of_several_fenced_calls_is_recovered
  - functions/src/llm/provider/ollama/fenced_non_tool_json_is_not_recovered
  - functions/src/llm/provider/ollama/prose_is_never_mistaken_for_a_tool_call
  - functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block
  - functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block
---

# Signature

`fn bash_tool() -> Tool`

# Called by

- [tool_call_printed_as_content_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_printed_as_content_is_recovered.md)
- [tool_call_in_a_json_fence_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_in_a_json_fence_is_recovered.md)
- [tool_call_in_a_fence_embedded_in_prose_is_recovered](../../../../../functions/src/llm/provider/ollama/tool_call_in_a_fence_embedded_in_prose_is_recovered.md)
- [first_of_several_fenced_calls_is_recovered](../../../../../functions/src/llm/provider/ollama/first_of_several_fenced_calls_is_recovered.md)
- [fenced_non_tool_json_is_not_recovered](../../../../../functions/src/llm/provider/ollama/fenced_non_tool_json_is_not_recovered.md)
- [prose_is_never_mistaken_for_a_tool_call](../../../../../functions/src/llm/provider/ollama/prose_is_never_mistaken_for_a_tool_call.md)
- [recovered_tool_call_becomes_a_tool_use_block](../../../../../functions/src/llm/provider/ollama/recovered_tool_call_becomes_a_tool_use_block.md)
- [fenced_call_in_prose_becomes_a_tool_use_block](../../../../../functions/src/llm/provider/ollama/fenced_call_in_prose_becomes_a_tool_use_block.md)
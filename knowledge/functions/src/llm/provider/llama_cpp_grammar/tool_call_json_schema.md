---
type: Rust Function
title: tool_call_json_schema
resource: src/llm/provider/llama_cpp_grammar.rs#L88-L105
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler
  - functions/src/llm/provider/llama_cpp_grammar/schema_has_one_variant_per_offered_tool
  - functions/src/llm/provider/llama_cpp_grammar/each_variant_pins_the_tool_name_and_embeds_its_input_schema
  - functions/src/llm/provider/llama_cpp_grammar/empty_tool_list_produces_an_empty_oneof
---

# Signature

`pub fn tool_call_json_schema(tools: &[Tool]) -> serde_json::Value`

# Called by

- [build_tool_call_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [schema_has_one_variant_per_offered_tool](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_has_one_variant_per_offered_tool.md)
- [each_variant_pins_the_tool_name_and_embeds_its_input_schema](../../../../../functions/src/llm/provider/llama_cpp_grammar/each_variant_pins_the_tool_name_and_embeds_its_input_schema.md)
- [empty_tool_list_produces_an_empty_oneof](../../../../../functions/src/llm/provider/llama_cpp_grammar/empty_tool_list_produces_an_empty_oneof.md)
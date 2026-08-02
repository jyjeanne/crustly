---
type: Rust Module
title: llama_cpp_grammar
resource: src/llm/provider/llama_cpp_grammar.rs#L1-L231
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/super-types-tool
  - external/llama-cpp-2-sampling-llamasampler
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [tool_call_json_schema](../../../../functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema.md)
- [build_parser_factory](../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)
- [build_tool_call_sampler](../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [bash_tool](../../../../functions/src/llm/provider/llama_cpp_grammar/bash_tool.md)
- [read_tool](../../../../functions/src/llm/provider/llama_cpp_grammar/read_tool.md)
- [schema_has_one_variant_per_offered_tool](../../../../functions/src/llm/provider/llama_cpp_grammar/schema_has_one_variant_per_offered_tool.md)
- [each_variant_pins_the_tool_name_and_embeds_its_input_schema](../../../../functions/src/llm/provider/llama_cpp_grammar/each_variant_pins_the_tool_name_and_embeds_its_input_schema.md)
- [empty_tool_list_produces_an_empty_oneof](../../../../functions/src/llm/provider/llama_cpp_grammar/empty_tool_list_produces_an_empty_oneof.md)
- [schema_compiles_into_a_working_sampler](../../../../functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler.md)
- [one_factory_builds_multiple_independent_grammars](../../../../functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars.md)

# Imports

- `super::types::Tool`
- `llama_cpp_2::sampling::LlamaSampler`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
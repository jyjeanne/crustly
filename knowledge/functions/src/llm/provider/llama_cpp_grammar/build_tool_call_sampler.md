---
type: Rust Function
title: build_tool_call_sampler
resource: src/llm/provider/llama_cpp_grammar.rs#L128-L142
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema
  called_by:
  - functions/src/llm/provider/llama_cpp/try_build_constrained_sampler
  - functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler
  - functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars
---

# Signature

`pub fn build_tool_call_sampler( factory: &llguidance::ParserFactory, tools: &[Tool], ) -> std::result::Result<LlamaSampler, String>`

# Calls

- [tool_call_json_schema](../../../../../functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema.md)

# Called by

- [try_build_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler.md)
- [schema_compiles_into_a_working_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler.md)
- [one_factory_builds_multiple_independent_grammars](../../../../../functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars.md)
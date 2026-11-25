---
type: Rust Function
title: build_tool_call_sampler
resource: src/llm/provider/llama_cpp_grammar.rs#L128-L142
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_tool_call_sampler( factory: &llguidance::ParserFactory, tools: &[Tool], ) -> std::result::Result<LlamaSampler, String>`

# Calls

- [tool_call_json_schema](../../../../../functions/src/llm/provider/llama_cpp_grammar/tool_call_json_schema.md)

# Called by

- [try_build_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler.md)
- [schema_compiles_into_a_working_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler.md)
- [one_factory_builds_multiple_independent_grammars](../../../../../functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars.md)
---
type: Rust Function
title: read_tool
resource: src/llm/provider/llama_cpp_grammar.rs#L160-L170
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_grammar/schema_has_one_variant_per_offered_tool
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

`fn read_tool() -> Tool`

# Called by

- [schema_has_one_variant_per_offered_tool](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_has_one_variant_per_offered_tool.md)
- [schema_compiles_into_a_working_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/schema_compiles_into_a_working_sampler.md)
- [one_factory_builds_multiple_independent_grammars](../../../../../functions/src/llm/provider/llama_cpp_grammar/one_factory_builds_multiple_independent_grammars.md)
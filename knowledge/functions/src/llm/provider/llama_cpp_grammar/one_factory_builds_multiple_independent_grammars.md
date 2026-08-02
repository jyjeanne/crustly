---
type: Rust Function
title: one_factory_builds_multiple_independent_grammars
resource: src/llm/provider/llama_cpp_grammar.rs#L222-L230
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_grammar/build_parser_factory
  - functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler
  - functions/src/llm/provider/llama_cpp_grammar/read_tool
---

# Signature

`fn one_factory_builds_multiple_independent_grammars()`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)
- [build_tool_call_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [read_tool](../../../../../functions/src/llm/provider/llama_cpp_grammar/read_tool.md)
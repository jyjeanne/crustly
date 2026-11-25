---
type: Rust Function
title: schema_compiles_into_a_working_sampler
resource: src/llm/provider/llama_cpp_grammar.rs#L206-L215
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_grammar/build_parser_factory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_grammar/read_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn schema_compiles_into_a_working_sampler()`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)
- [build_tool_call_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [read_tool](../../../../../functions/src/llm/provider/llama_cpp_grammar/read_tool.md)
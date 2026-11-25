---
type: Rust Function
title: test_grammar_env
resource: src/llm/provider/llama_cpp.rs#L1782-L1786
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
  called_by:
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_grammar_env() -> ToolCallGrammarEnv`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)

# Called by

- [try_build_constrained_sampler_succeeds_for_a_valid_tool_schema](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema.md)
- [try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay.md)
---
type: Rust Function
title: test_grammar_env
resource: src/llm/provider/llama_cpp.rs#L1681-L1685
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_grammar/build_parser_factory
  called_by:
  - functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema
  - functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay
---

# Signature

`fn test_grammar_env() -> ToolCallGrammarEnv`

# Calls

- [build_parser_factory](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_parser_factory.md)

# Called by

- [try_build_constrained_sampler_succeeds_for_a_valid_tool_schema](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema.md)
- [try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay.md)
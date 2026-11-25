---
type: Rust Function
title: try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay
resource: src/llm/provider/llama_cpp.rs#L1818-L1838
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/test_grammar_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay()`

# Calls

- [test_grammar_env](../../../../../functions/src/llm/provider/llama_cpp/test_grammar_env.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [default](../../../../../functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default.md)
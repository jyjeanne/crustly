---
type: Rust Function
title: try_build_constrained_sampler
resource: src/llm/provider/llama_cpp.rs#L1284-L1320
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/build_sampler
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn try_build_constrained_sampler( n_vocab: i32, grammar_env: &ToolCallGrammarEnv, offered_tools: &[Tool], generated_tokens: &[LlamaToken], defaults: &SamplingDefaults, request: &LLMRequest, default_seed: Option<u32>, ) -> Option<LlamaSampler>`

# Calls

- [build_tool_call_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [build_sampler](../../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)
---
type: Rust Function
title: try_build_constrained_sampler
resource: src/llm/provider/llama_cpp.rs#L1189-L1223
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler
  - functions/src/llm/provider/llama_cpp/build_sampler
  - functions/src/config/secrets/SecretString/len
---

# Signature

`fn try_build_constrained_sampler( grammar_env: &ToolCallGrammarEnv, offered_tools: &[Tool], generated_tokens: &[LlamaToken], defaults: &SamplingDefaults, request: &LLMRequest, default_seed: Option<u32>, ) -> Option<LlamaSampler>`

# Calls

- [build_tool_call_sampler](../../../../../functions/src/llm/provider/llama_cpp_grammar/build_tool_call_sampler.md)
- [build_sampler](../../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
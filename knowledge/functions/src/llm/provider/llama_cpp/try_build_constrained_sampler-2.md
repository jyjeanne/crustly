---
type: Rust Function
title: try_build_constrained_sampler
resource: src/llm/provider/llama_cpp.rs#L1226-L1235
visibility: private
generated:
  by: okf-rs/0.3.0
---

# Signature

`fn try_build_constrained_sampler( _grammar_env: &ToolCallGrammarEnv, _offered_tools: &[Tool], _generated_tokens: &[LlamaToken], _defaults: &SamplingDefaults, _request: &LLMRequest, _default_seed: Option<u32>, ) -> Option<LlamaSampler>`
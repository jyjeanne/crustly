---
type: Rust Function
title: try_build_constrained_sampler
resource: src/llm/provider/llama_cpp.rs#L1324-L1334
visibility: private
generated:
  by: okf-rs/0.7.0
---

# Signature

`fn try_build_constrained_sampler( _n_vocab: i32, _grammar_env: &ToolCallGrammarEnv, _offered_tools: &[Tool], _generated_tokens: &[LlamaToken], _defaults: &SamplingDefaults, _request: &LLMRequest, _default_seed: Option<u32>, ) -> Option<LlamaSampler>`
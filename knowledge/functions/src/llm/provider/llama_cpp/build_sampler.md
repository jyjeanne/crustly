---
type: Rust Function
title: build_sampler
resource: src/llm/provider/llama_cpp.rs#L1163-L1212
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/prepare_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_sampler( n_vocab: i32, defaults: &SamplingDefaults, request: &LLMRequest, default_seed: Option<u32>, seed_offset: u32, grammar: Option<LlamaSampler>, ) -> LlamaSampler`

# Called by

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)
- [try_build_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler.md)
- [build_sampler_seed_offset_changes_the_resolved_seed](../../../../../functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed.md)
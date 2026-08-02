---
type: Rust Function
title: build_sampler
resource: src/llm/provider/llama_cpp.rs#L1071-L1118
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp/prepare_generation
  - functions/src/llm/provider/llama_cpp/try_build_constrained_sampler
  - functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed
---

# Signature

`fn build_sampler( defaults: &SamplingDefaults, request: &LLMRequest, default_seed: Option<u32>, seed_offset: u32, grammar: Option<LlamaSampler>, ) -> LlamaSampler`

# Called by

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)
- [try_build_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler.md)
- [build_sampler_seed_offset_changes_the_resolved_seed](../../../../../functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed.md)
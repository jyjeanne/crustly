---
type: Rust Function
title: build_sampler_seed_offset_changes_the_resolved_seed
resource: src/llm/provider/llama_cpp.rs#L1618-L1628
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/build_sampler
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_sampler_seed_offset_changes_the_resolved_seed()`

# Calls

- [default](../../../../../functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [build_sampler](../../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)
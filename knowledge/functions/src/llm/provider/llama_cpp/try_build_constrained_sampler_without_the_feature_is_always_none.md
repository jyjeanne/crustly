---
type: Rust Function
title: try_build_constrained_sampler_without_the_feature_is_always_none
resource: src/llm/provider/llama_cpp.rs#L1842-L1856
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn try_build_constrained_sampler_without_the_feature_is_always_none()`

# Calls

- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [default](../../../../../functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default.md)
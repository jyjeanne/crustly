---
type: Rust Method
title: default
resource: src/llm/provider/llama_cpp.rs#L71-L78
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/sampling_defaults_match_documented_values
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_without_the_feature_is_always_none
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [sampling_defaults_match_documented_values](../../../../../../../functions/src/llm/provider/llama_cpp/sampling_defaults_match_documented_values.md)
- [build_sampler_seed_offset_changes_the_resolved_seed](../../../../../../../functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed.md)
- [try_build_constrained_sampler_succeeds_for_a_valid_tool_schema](../../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema.md)
- [try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay](../../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay.md)
- [try_build_constrained_sampler_without_the_feature_is_always_none](../../../../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_without_the_feature_is_always_none.md)
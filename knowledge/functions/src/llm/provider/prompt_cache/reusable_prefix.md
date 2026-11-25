---
type: Rust Function
title: reusable_prefix
resource: src/llm/provider/prompt_cache.rs#L20-L27
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/prefill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/prompt_cache/a_growing_prompt_reuses_everything_already_cached
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/prompt_cache/a_changed_prompt_reuses_only_the_common_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/prompt_cache/the_last_prompt_token_is_always_decoded_again
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn reusable_prefix<T: PartialEq>(cached: &[T], prompt: &[T]) -> usize`

# Called by

- [prefill](../../../../../functions/src/llm/provider/llama_cpp/prefill.md)
- [a_growing_prompt_reuses_everything_already_cached](../../../../../functions/src/llm/provider/prompt_cache/a_growing_prompt_reuses_everything_already_cached.md)
- [a_changed_prompt_reuses_only_the_common_prefix](../../../../../functions/src/llm/provider/prompt_cache/a_changed_prompt_reuses_only_the_common_prefix.md)
- [the_last_prompt_token_is_always_decoded_again](../../../../../functions/src/llm/provider/prompt_cache/the_last_prompt_token_is_always_decoded_again.md)
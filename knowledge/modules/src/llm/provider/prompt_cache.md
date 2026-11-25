---
type: Rust Module
title: prompt_cache
resource: src/llm/provider/prompt_cache.rs#L1-L79
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-ops-range
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [reusable_prefix](../../../../functions/src/llm/provider/prompt_cache/reusable_prefix.md)
- [prefill_batches](../../../../functions/src/llm/provider/prompt_cache/prefill_batches.md)
- [a_growing_prompt_reuses_everything_already_cached](../../../../functions/src/llm/provider/prompt_cache/a_growing_prompt_reuses_everything_already_cached.md)
- [a_changed_prompt_reuses_only_the_common_prefix](../../../../functions/src/llm/provider/prompt_cache/a_changed_prompt_reuses_only_the_common_prefix.md)
- [the_last_prompt_token_is_always_decoded_again](../../../../functions/src/llm/provider/prompt_cache/the_last_prompt_token_is_always_decoded_again.md)
- [prefill_is_split_into_batches_no_larger_than_n_batch](../../../../functions/src/llm/provider/prompt_cache/prefill_is_split_into_batches_no_larger_than_n_batch.md)

# Imports

- `std::ops::Range`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
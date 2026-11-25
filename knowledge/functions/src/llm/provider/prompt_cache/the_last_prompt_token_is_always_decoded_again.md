---
type: Rust Function
title: the_last_prompt_token_is_always_decoded_again
resource: src/llm/provider/prompt_cache.rs#L60-L67
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/prompt_cache/reusable_prefix
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn the_last_prompt_token_is_always_decoded_again()`

# Calls

- [reusable_prefix](../../../../../functions/src/llm/provider/prompt_cache/reusable_prefix.md)
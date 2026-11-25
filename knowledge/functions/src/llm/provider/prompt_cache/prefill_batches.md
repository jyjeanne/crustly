---
type: Rust Function
title: prefill_batches
resource: src/llm/provider/prompt_cache.rs#L33-L39
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/prefill
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/prompt_cache/prefill_is_split_into_batches_no_larger_than_n_batch
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn prefill_batches(start: usize, end: usize, n_batch: usize) -> Vec<Range<usize>>`

# Called by

- [prefill](../../../../../functions/src/llm/provider/llama_cpp/prefill.md)
- [prefill_is_split_into_batches_no_larger_than_n_batch](../../../../../functions/src/llm/provider/prompt_cache/prefill_is_split_into_batches_no_larger_than_n_batch.md)
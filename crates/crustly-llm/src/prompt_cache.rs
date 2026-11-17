//! Planning the prompt prefill of a model that keeps its KV cache between
//! requests (the in-process llama.cpp provider).
//!
//! Every round of the agent loop sends the same prompt as the previous round
//! plus the new turns: system prompt, tool schemas and history are an
//! unchanged prefix. Re-processing that prefix every round is the slowest
//! step of local inference. Instead, the provider remembers which tokens its
//! KV cache holds, keeps the longest common prefix with the new prompt, and
//! decodes only the rest.
//!
//! This module is the pure part of that, kept free of llama.cpp types so it
//! is testable without building llama.cpp.

use std::ops::Range;

/// How many leading tokens of `prompt` can stay in a KV cache that holds
/// `cached`: the length of their common prefix, but never the whole prompt.
/// Sampling reads the logits of the last prompt token, so that token is
/// always decoded again.
pub fn reusable_prefix<T: PartialEq>(cached: &[T], prompt: &[T]) -> usize {
    let common = cached
        .iter()
        .zip(prompt)
        .take_while(|(cached, new)| cached == new)
        .count();
    common.min(prompt.len().saturating_sub(1))
}

/// The positions `start..end` split into batches of at most `n_batch`
/// tokens. llama.cpp aborts the whole process (`GGML_ASSERT`) when a single
/// decode carries more tokens than the context's batch size, so a long
/// prompt must be decoded in pieces.
pub fn prefill_batches(start: usize, end: usize, n_batch: usize) -> Vec<Range<usize>> {
    let n_batch = n_batch.max(1);
    (start..end)
        .step_by(n_batch)
        .map(|from| from..(from + n_batch).min(end))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_growing_prompt_reuses_everything_already_cached() {
        let cached = [1, 2, 3, 4, 5];
        let prompt = [1, 2, 3, 4, 5, 6, 7];
        assert_eq!(reusable_prefix(&cached, &prompt), 5);
    }

    #[test]
    fn a_changed_prompt_reuses_only_the_common_prefix() {
        assert_eq!(reusable_prefix(&[1, 2, 3, 4], &[1, 2, 9, 4, 5]), 2);
        assert_eq!(reusable_prefix(&[7, 8], &[1, 2, 3]), 0);
        assert_eq!(reusable_prefix::<i32>(&[], &[1, 2, 3]), 0);
    }

    #[test]
    fn the_last_prompt_token_is_always_decoded_again() {
        // Same prompt twice: everything but the last token is reused.
        assert_eq!(reusable_prefix(&[1, 2, 3], &[1, 2, 3]), 2);
        // Prompt shorter than the cache (generated tokens follow it).
        assert_eq!(reusable_prefix(&[1, 2, 3, 4, 5], &[1, 2, 3]), 2);
        assert_eq!(reusable_prefix(&[1], &[1]), 0);
        assert_eq!(reusable_prefix::<i32>(&[1], &[]), 0);
    }

    #[test]
    fn prefill_is_split_into_batches_no_larger_than_n_batch() {
        assert_eq!(
            prefill_batches(0, 5000, 2048),
            vec![0..2048, 2048..4096, 4096..5000]
        );
        assert_eq!(prefill_batches(4000, 4100, 2048), vec![4000..4100]);
        assert_eq!(prefill_batches(10, 10, 2048), Vec::<Range<usize>>::new());
        assert_eq!(prefill_batches(0, 3, 0), vec![0..1, 1..2, 2..3]);
    }
}

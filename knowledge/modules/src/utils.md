---
type: Rust Module
title: utils
resource: src/utils/mod.rs#L1-L45
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/pub-use-retry-retry-retry-with-check-retryconfig-retryableerror
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-truncate-at-char-boundary
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [truncate_at_char_boundary](../../functions/src/utils/truncate_at_char_boundary.md)
- [truncate_ascii_at_exact_boundary](../../functions/src/utils/truncate_ascii_at_exact_boundary.md)
- [truncate_multibyte_does_not_panic](../../functions/src/utils/truncate_multibyte_does_not_panic.md)
- [truncate_empty_and_zero](../../functions/src/utils/truncate_empty_and_zero.md)

# Imports

- `pub use retry::{retry, retry_with_check, RetryConfig, RetryableError}`
- `super::truncate_at_char_boundary`

# Member of

- [crustly](../../packages/crustly.md)
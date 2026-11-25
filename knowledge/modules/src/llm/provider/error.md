---
type: Rust Module
title: error
resource: src/llm/provider/error.rs#L1-L196
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/thiserror-error
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

- [ProviderError](../../../../classes/src/llm/provider/error/ProviderError.md)
- [is_retryable](../../../../functions/src/llm/provider/error/ProviderError/is_retryable.md)
- [is_context_overflow](../../../../functions/src/llm/provider/error/ProviderError/is_context_overflow.md)
- [status_code](../../../../functions/src/llm/provider/error/ProviderError/status_code.md)
- [mentions_context_overflow](../../../../functions/src/llm/provider/error/mentions_context_overflow.md)
- [recognises_context_overflow_from_each_provider](../../../../functions/src/llm/provider/error/recognises_context_overflow_from_each_provider.md)
- [test_error_retryable](../../../../functions/src/llm/provider/error/test_error_retryable.md)
- [test_status_code](../../../../functions/src/llm/provider/error/test_status_code.md)

# Imports

- `thiserror::Error`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
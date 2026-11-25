---
type: Rust Function
title: extract_retry_after
resource: src/llm/provider/retry.rs#L196-L216
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/retry/parse_retry_seconds
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/retry/test_extract_retry_after
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_retry_after(error: &ProviderError) -> Option<Duration>`

# Calls

- [parse_retry_seconds](../../../../../functions/src/llm/provider/retry/parse_retry_seconds.md)

# Called by

- [test_extract_retry_after](../../../../../functions/src/llm/provider/retry/test_extract_retry_after.md)
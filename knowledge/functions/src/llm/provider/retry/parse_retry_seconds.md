---
type: Rust Function
title: parse_retry_seconds
resource: src/llm/provider/retry.rs#L219-L244
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/retry/extract_retry_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/retry/test_parse_retry_seconds
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_retry_seconds(msg: &str) -> Option<u64>`

# Called by

- [extract_retry_after](../../../../../functions/src/llm/provider/retry/extract_retry_after.md)
- [test_parse_retry_seconds](../../../../../functions/src/llm/provider/retry/test_parse_retry_seconds.md)
---
type: Rust Function
title: parse_retry_seconds
resource: src/llm/provider/retry.rs#L219-L244
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/retry/extract_retry_after
---

# Signature

`fn parse_retry_seconds(msg: &str) -> Option<u64>`

# Called by

- [extract_retry_after](../../../../../functions/src/llm/provider/retry/extract_retry_after.md)
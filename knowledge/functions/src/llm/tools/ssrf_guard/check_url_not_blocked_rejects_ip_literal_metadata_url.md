---
type: Rust Function
title: check_url_not_blocked_rejects_ip_literal_metadata_url
resource: src/llm/tools/ssrf_guard.rs#L228-L231
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_url_not_blocked_rejects_ip_literal_metadata_url()`

# Calls

- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
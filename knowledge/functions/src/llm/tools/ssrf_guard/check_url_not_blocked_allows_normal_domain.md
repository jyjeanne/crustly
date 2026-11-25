---
type: Rust Function
title: check_url_not_blocked_allows_normal_domain
resource: src/llm/tools/ssrf_guard.rs#L234-L236
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_url_not_blocked_allows_normal_domain()`

# Calls

- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
---
type: Rust Function
title: is_blocked_ipv6
resource: src/llm/tools/ssrf_guard.rs#L53-L74
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ipv4
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_blocked_ipv6(ip: &Ipv6Addr) -> bool`

# Calls

- [is_blocked_ipv4](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)

# Called by

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
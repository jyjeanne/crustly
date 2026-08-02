---
type: Rust Function
title: is_blocked_ipv4
resource: src/llm/tools/ssrf_guard.rs#L36-L54
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/ssrf_guard/is_blocked_ip
  - functions/src/llm/tools/ssrf_guard/is_blocked_ipv6
---

# Signature

`fn is_blocked_ipv4(ip: &Ipv4Addr) -> bool`

# Called by

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [is_blocked_ipv6](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv6.md)
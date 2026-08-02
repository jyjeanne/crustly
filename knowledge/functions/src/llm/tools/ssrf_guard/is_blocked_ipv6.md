---
type: Rust Function
title: is_blocked_ipv6
resource: src/llm/tools/ssrf_guard.rs#L56-L77
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/ssrf_guard/is_blocked_ipv4
  called_by:
  - functions/src/llm/tools/ssrf_guard/is_blocked_ip
---

# Signature

`fn is_blocked_ipv6(ip: &Ipv6Addr) -> bool`

# Calls

- [is_blocked_ipv4](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)

# Called by

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
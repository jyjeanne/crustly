---
type: Rust Function
title: is_blocked_ip
resource: src/llm/tools/ssrf_guard.rs#L29-L34
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/ssrf_guard/is_blocked_ipv4
  - functions/src/llm/tools/ssrf_guard/is_blocked_ipv6
  called_by:
  - functions/src/llm/tools/ssrf_guard/SsrfSafeResolver/resolve/resolve
  - functions/src/llm/tools/ssrf_guard/check_url_not_blocked
---

# Signature

`pub fn is_blocked_ip(ip: &IpAddr) -> bool`

# Calls

- [is_blocked_ipv4](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)
- [is_blocked_ipv6](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv6.md)

# Called by

- [resolve](../../../../../functions/src/llm/tools/ssrf_guard/SsrfSafeResolver/resolve/resolve.md)
- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
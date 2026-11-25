---
type: Rust Function
title: is_blocked_ipv4
resource: src/llm/tools/ssrf_guard.rs#L36-L51
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ipv6
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_blocked_ipv4(ip: &Ipv4Addr) -> bool`

# Calls

- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [is_blocked_ipv6](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv6.md)
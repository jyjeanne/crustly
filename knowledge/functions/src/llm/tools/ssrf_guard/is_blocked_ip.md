---
type: Rust Function
title: is_blocked_ip
resource: src/llm/tools/ssrf_guard.rs#L29-L34
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ipv4
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ipv6
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/ssrf_guard/SsrfSafeResolver/resolve/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/check_url_not_blocked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_loopback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_cloud_metadata_link_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_rfc1918_private_ranges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_carrier_grade_nat_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_ipv6_unique_local_and_link_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/blocks_ipv4_mapped_blocked_address
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/ssrf_guard/allows_public_addresses
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_blocked_ip(ip: &IpAddr) -> bool`

# Calls

- [is_blocked_ipv4](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)
- [is_blocked_ipv6](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv6.md)

# Called by

- [resolve](../../../../../functions/src/llm/tools/ssrf_guard/SsrfSafeResolver/resolve/resolve.md)
- [check_url_not_blocked](../../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [blocks_loopback](../../../../../functions/src/llm/tools/ssrf_guard/blocks_loopback.md)
- [blocks_cloud_metadata_link_local](../../../../../functions/src/llm/tools/ssrf_guard/blocks_cloud_metadata_link_local.md)
- [blocks_rfc1918_private_ranges](../../../../../functions/src/llm/tools/ssrf_guard/blocks_rfc1918_private_ranges.md)
- [blocks_carrier_grade_nat_range](../../../../../functions/src/llm/tools/ssrf_guard/blocks_carrier_grade_nat_range.md)
- [blocks_ipv6_unique_local_and_link_local](../../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv6_unique_local_and_link_local.md)
- [blocks_ipv4_mapped_blocked_address](../../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv4_mapped_blocked_address.md)
- [allows_public_addresses](../../../../../functions/src/llm/tools/ssrf_guard/allows_public_addresses.md)
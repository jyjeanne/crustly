---
type: Rust Module
title: ssrf_guard
resource: src/llm/tools/ssrf_guard.rs#L1-L291
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/reqwest-dns-addrs-name-resolve-resolving
  - external/std-net-ipaddr-ipv4addr-ipv6addr-socketaddr
  - external/std-sync-arc
  - external/super
  - external/tokio-io-asyncreadext-asyncwriteext
  member_of:
  - packages/crustly
---

# Contains

- [is_blocked_ip](../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [is_blocked_ipv4](../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv4.md)
- [is_blocked_ipv6](../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ipv6.md)
- [SsrfSafeResolver](../../../../classes/src/llm/tools/ssrf_guard/SsrfSafeResolver.md)
- [resolve](../../../../functions/src/llm/tools/ssrf_guard/SsrfSafeResolver/resolve/resolve.md)
- [check_url_not_blocked](../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked.md)
- [guard](../../../../functions/src/llm/tools/ssrf_guard/guard.md)
- [checked_redirect_policy](../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [blocks_loopback](../../../../functions/src/llm/tools/ssrf_guard/blocks_loopback.md)
- [blocks_cloud_metadata_link_local](../../../../functions/src/llm/tools/ssrf_guard/blocks_cloud_metadata_link_local.md)
- [blocks_rfc1918_private_ranges](../../../../functions/src/llm/tools/ssrf_guard/blocks_rfc1918_private_ranges.md)
- [blocks_carrier_grade_nat_range](../../../../functions/src/llm/tools/ssrf_guard/blocks_carrier_grade_nat_range.md)
- [blocks_ipv6_unique_local_and_link_local](../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv6_unique_local_and_link_local.md)
- [blocks_ipv4_mapped_blocked_address](../../../../functions/src/llm/tools/ssrf_guard/blocks_ipv4_mapped_blocked_address.md)
- [allows_public_addresses](../../../../functions/src/llm/tools/ssrf_guard/allows_public_addresses.md)
- [check_url_not_blocked_rejects_ip_literal_metadata_url](../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked_rejects_ip_literal_metadata_url.md)
- [check_url_not_blocked_allows_normal_domain](../../../../functions/src/llm/tools/ssrf_guard/check_url_not_blocked_allows_normal_domain.md)
- [checked_redirect_policy_blocks_redirect_to_blocked_address](../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy_blocks_redirect_to_blocked_address.md)

# Imports

- `reqwest::dns::{Addrs, Name, Resolve, Resolving}`
- `std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr}`
- `std::sync::Arc`
- `super::*`
- `tokio::io::{AsyncReadExt, AsyncWriteExt}`

# Member of

- [crustly](../../../../packages/crustly.md)
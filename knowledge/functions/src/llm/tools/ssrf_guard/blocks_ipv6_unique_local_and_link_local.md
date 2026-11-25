---
type: Rust Function
title: blocks_ipv6_unique_local_and_link_local
resource: src/llm/tools/ssrf_guard.rs#L208-L211
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn blocks_ipv6_unique_local_and_link_local()`

# Calls

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
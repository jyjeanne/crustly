---
type: Rust Function
title: blocks_cloud_metadata_link_local
resource: src/llm/tools/ssrf_guard.rs#L189-L191
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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

`fn blocks_cloud_metadata_link_local()`

# Calls

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
---
type: Rust Function
title: blocks_carrier_grade_nat_range
resource: src/llm/tools/ssrf_guard.rs#L201-L205
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

`fn blocks_carrier_grade_nat_range()`

# Calls

- [is_blocked_ip](../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
- [parse](../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
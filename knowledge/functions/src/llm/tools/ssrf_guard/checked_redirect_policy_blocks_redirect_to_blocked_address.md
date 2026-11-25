---
type: Rust Function
title: checked_redirect_policy_blocks_redirect_to_blocked_address
resource: src/llm/tools/ssrf_guard.rs#L250-L287
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/checked_redirect_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn checked_redirect_policy_blocks_redirect_to_blocked_address()`

# Calls

- [checked_redirect_policy](../../../../../functions/src/llm/tools/ssrf_guard/checked_redirect_policy.md)
- [build](../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
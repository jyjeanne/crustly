---
type: Rust Method
title: resolve
resource: src/llm/tools/ssrf_guard.rs#L84-L123
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/ssrf_guard/is_blocked_ip
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve(&self, name: Name) -> Resolving`

# Calls

- [is_blocked_ip](../../../../../../../functions/src/llm/tools/ssrf_guard/is_blocked_ip.md)
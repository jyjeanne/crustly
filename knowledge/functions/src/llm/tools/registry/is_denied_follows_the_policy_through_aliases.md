---
type: Rust Function
title: is_denied_follows_the_policy_through_aliases
resource: src/llm/tools/registry.rs#L571-L589
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/set_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/DenyToolRule/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/is_denied
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_denied_follows_the_policy_through_aliases()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [set_policy](../../../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [new](../../../../../functions/src/llm/tools/sandbox/DenyToolRule/new.md)
- [is_denied](../../../../../functions/src/llm/tools/registry/ToolRegistry/is_denied.md)
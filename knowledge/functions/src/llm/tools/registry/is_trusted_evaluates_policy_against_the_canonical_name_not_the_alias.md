---
type: Rust Function
title: is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
resource: src/llm/tools/registry.rs#L674-L691
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
  - target: functions/src/llm/tools/sandbox/AllowToolRule/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/is_trusted
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [set_policy](../../../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [new](../../../../../functions/src/llm/tools/sandbox/AllowToolRule/new.md)
- [is_trusted](../../../../../functions/src/llm/tools/registry/ToolRegistry/is_trusted.md)
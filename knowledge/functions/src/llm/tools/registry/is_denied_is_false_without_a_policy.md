---
type: Rust Function
title: is_denied_is_false_without_a_policy
resource: src/llm/tools/registry.rs#L592-L595
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
  - target: functions/src/llm/tools/registry/ToolRegistry/is_denied
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_denied_is_false_without_a_policy()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [is_denied](../../../../../functions/src/llm/tools/registry/ToolRegistry/is_denied.md)
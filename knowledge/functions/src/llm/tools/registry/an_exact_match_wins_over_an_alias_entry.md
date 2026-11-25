---
type: Rust Function
title: an_exact_match_wins_over_an_alias_entry
resource: src/llm/tools/registry.rs#L540-L554
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn an_exact_match_wins_over_an_alias_entry()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [get](../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
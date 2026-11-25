---
type: Rust Function
title: execute_reports_not_found_using_the_original_unresolved_name
resource: src/llm/tools/registry.rs#L620-L636
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
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute_reports_not_found_using_the_original_unresolved_name()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [execute](../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)
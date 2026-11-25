---
type: Rust Function
title: test_execute_requires_approval
resource: src/llm/tools/registry.rs#L450-L469
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
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_execute_requires_approval()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [execute](../../../../../functions/src/llm/tools/registry/ToolRegistry/execute.md)
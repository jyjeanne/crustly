---
type: Rust Method
title: description
resource: src/tui/error.rs#L173-L215
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorSeverity/prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorSeverity/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/error/ErrorCategory/name
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn description(&self) -> Vec<String>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [prefix](../../../../../functions/src/tui/error/ErrorSeverity/prefix.md)
- [name](../../../../../functions/src/tui/error/ErrorSeverity/name.md)
- [name](../../../../../functions/src/tui/error/ErrorCategory/name.md)
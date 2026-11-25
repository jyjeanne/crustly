---
type: Rust Function
title: find_active_shell_operator
resource: src/llm/tools/sandbox.rs#L296-L328
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/bash/is_read_only_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn find_active_shell_operator(cmd: &str) -> Option<&'static str>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate.md)
---
type: Rust Function
title: find_active_shell_operator
resource: src/llm/tools/sandbox.rs#L296-L328
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  called_by:
  - functions/src/llm/tools/bash/is_read_only_command
  - functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate
---

# Signature

`pub fn find_active_shell_operator(cmd: &str) -> Option<&'static str>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
- [evaluate](../../../../../functions/src/llm/tools/sandbox/BashCommandAllowlist/permissionpolicy/evaluate.md)
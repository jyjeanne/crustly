---
type: Rust Function
title: is_read_only_command
resource: src/llm/tools/bash.rs#L123-L239
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/find_active_shell_operator
  - functions/src/tui/events/EventHandler/next
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/tools/bash/BashTool/tool/execute
---

# Signature

`fn is_read_only_command(command: &str) -> bool`

# Calls

- [find_active_shell_operator](../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
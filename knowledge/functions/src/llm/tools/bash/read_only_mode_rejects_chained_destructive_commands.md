---
type: Rust Function
title: read_only_mode_rejects_chained_destructive_commands
resource: src/llm/tools/bash.rs#L680-L686
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/bash/is_read_only_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_only_mode_rejects_chained_destructive_commands()`

# Calls

- [is_read_only_command](../../../../../functions/src/llm/tools/bash/is_read_only_command.md)
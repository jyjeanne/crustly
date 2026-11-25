---
type: Rust Function
title: is_read_only_command
resource: src/llm/tools/bash.rs#L123-L239
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/find_active_shell_operator
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/read_only_mode_rejects_chained_destructive_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/read_only_mode_rejects_network_fetch_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/read_only_mode_allows_simple_safe_commands
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/read_only_mode_rejects_mutating_find_flags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/read_only_mode_rejects_git_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_read_only_command(command: &str) -> bool`

# Calls

- [find_active_shell_operator](../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [read_only_mode_rejects_chained_destructive_commands](../../../../../functions/src/llm/tools/bash/read_only_mode_rejects_chained_destructive_commands.md)
- [read_only_mode_rejects_network_fetch_tools](../../../../../functions/src/llm/tools/bash/read_only_mode_rejects_network_fetch_tools.md)
- [read_only_mode_allows_simple_safe_commands](../../../../../functions/src/llm/tools/bash/read_only_mode_allows_simple_safe_commands.md)
- [read_only_mode_rejects_mutating_find_flags](../../../../../functions/src/llm/tools/bash/read_only_mode_rejects_mutating_find_flags.md)
- [read_only_mode_rejects_git_config](../../../../../functions/src/llm/tools/bash/read_only_mode_rejects_git_config.md)
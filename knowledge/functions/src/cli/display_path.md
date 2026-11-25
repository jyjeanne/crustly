---
type: Rust Function
title: display_path
resource: src/cli/mod.rs#L1018-L1023
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/memory_status_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/display_path_drops_the_windows_verbatim_prefix
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn display_path(path: &std::path::Path) -> String`

# Called by

- [memory_status_lines](../../../functions/src/cli/memory_status_lines.md)
- [cmd_memory](../../../functions/src/cli/cmd_memory.md)
- [display_path_drops_the_windows_verbatim_prefix](../../../functions/src/cli/display_path_drops_the_windows_verbatim_prefix.md)
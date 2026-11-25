---
type: Rust Function
title: windows_resolves_a_posix_shell_not_cmd
resource: src/llm/tools/bash.rs#L449-L461
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/bash/resolve_shell
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn windows_resolves_a_posix_shell_not_cmd()`

# Calls

- [resolve_shell](../../../../../functions/src/llm/tools/bash/resolve_shell.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
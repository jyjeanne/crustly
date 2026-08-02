---
type: Rust Function
title: resolve_shell
resource: src/llm/tools/bash.rs#L37-L77
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/bash/BashTool/tool/execute
  - functions/src/llm/tools/bash/windows_resolves_a_posix_shell_not_cmd
---

# Signature

`fn resolve_shell() -> (String, &'static str)`

# Called by

- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [windows_resolves_a_posix_shell_not_cmd](../../../../../functions/src/llm/tools/bash/windows_resolves_a_posix_shell_not_cmd.md)
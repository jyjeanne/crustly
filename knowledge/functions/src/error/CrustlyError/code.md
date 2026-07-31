---
type: Rust Method
title: code
resource: src/error.rs#L56-L63
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/tools/bash/BashTool/tool/execute
  - functions/src/llm/tools/code_exec/CodeExecTool/tool/execute
  - functions/src/llm/tools/powershell/PowerShellTool/tool/execute
---

# Signature

`pub fn code(&self) -> Option<ErrorCode>`

# Called by

- [execute](../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
- [execute](../../../../functions/src/llm/tools/code_exec/CodeExecTool/tool/execute.md)
- [execute](../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
---
type: Rust Function
title: bash_runs_posix_in_the_requested_working_directory
resource: src/llm/tools/bash.rs#L464-L497
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn bash_runs_posix_in_the_requested_working_directory()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
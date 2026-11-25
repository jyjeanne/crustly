---
type: Rust Function
title: test_bash_timeout
resource: src/llm/tools/bash.rs#L553-L567
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/bash/BashTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_bash_timeout()`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_auto_approve](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)
- [with_timeout](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_timeout.md)
- [execute](../../../../../functions/src/llm/tools/bash/BashTool/tool/execute.md)
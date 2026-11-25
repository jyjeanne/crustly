---
type: Rust Function
title: test_regex_false_still_searches_literally
resource: src/llm/tools/grep.rs#L413-L436
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/grep/GrepTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_regex_false_still_searches_literally()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)
- [execute](../../../../../functions/src/llm/tools/grep/GrepTool/tool/execute.md)
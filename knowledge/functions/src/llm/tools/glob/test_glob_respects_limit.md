---
type: Rust Function
title: test_glob_respects_limit
resource: src/llm/tools/glob.rs#L279-L293
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
  - target: functions/src/llm/tools/glob/GlobTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_glob_respects_limit()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
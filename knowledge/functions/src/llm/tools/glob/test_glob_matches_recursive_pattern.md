---
type: Rust Function
title: test_glob_matches_recursive_pattern
resource: src/llm/tools/glob.rs#L206-L229
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn test_glob_matches_recursive_pattern()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [execute](../../../../../functions/src/llm/tools/glob/GlobTool/tool/execute.md)
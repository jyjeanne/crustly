---
type: Rust Function
title: verification_detects_the_check_command_from_build_files
resource: src/llm/agent/middleware.rs#L737-L753
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
  - target: functions/src/llm/agent/middleware/Verification/detect_command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verification_detects_the_check_command_from_build_files()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [detect_command](../../../../../functions/src/llm/agent/middleware/Verification/detect_command.md)
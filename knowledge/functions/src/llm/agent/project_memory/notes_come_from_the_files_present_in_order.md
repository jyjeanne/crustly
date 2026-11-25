---
type: Rust Function
title: notes_come_from_the_files_present_in_order
resource: src/llm/agent/project_memory.rs#L277-L290
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
  - target: functions/src/llm/agent/project_memory/load_project_notes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn notes_come_from_the_files_present_in_order()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [load_project_notes](../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
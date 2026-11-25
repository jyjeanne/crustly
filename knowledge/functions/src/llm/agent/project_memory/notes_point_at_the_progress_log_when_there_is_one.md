---
type: Rust Function
title: notes_point_at_the_progress_log_when_there_is_one
resource: src/llm/agent/project_memory.rs#L314-L327
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/load_project_notes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn notes_point_at_the_progress_log_when_there_is_one()`

# Calls

- [append_progress](../../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [load_project_notes](../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
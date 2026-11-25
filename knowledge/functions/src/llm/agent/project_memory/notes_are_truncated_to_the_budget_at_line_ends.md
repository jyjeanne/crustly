---
type: Rust Function
title: notes_are_truncated_to_the_budget_at_line_ends
resource: src/llm/agent/project_memory.rs#L293-L311
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
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn notes_are_truncated_to_the_budget_at_line_ends()`

# Calls

- [write](../../../../../functions/src/memory/project/write.md)
- [load_project_notes](../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)
---
type: Rust Function
title: recent_work_gives_the_last_entries_oldest_first
resource: src/llm/agent/project_memory.rs#L395-L408
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/log_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn recent_work_gives_the_last_entries_oldest_first()`

# Calls

- [log_entry](../../../../../functions/src/llm/agent/project_memory/log_entry.md)
- [recent_work](../../../../../functions/src/llm/agent/project_memory/recent_work.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
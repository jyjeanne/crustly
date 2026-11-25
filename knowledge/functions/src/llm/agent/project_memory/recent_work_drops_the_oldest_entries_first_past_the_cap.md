---
type: Rust Function
title: recent_work_drops_the_oldest_entries_first_past_the_cap
resource: src/llm/agent/project_memory.rs#L417-L433
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/log_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
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

`fn recent_work_drops_the_oldest_entries_first_past_the_cap()`

# Calls

- [log_entry](../../../../../functions/src/llm/agent/project_memory/log_entry.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)
- [recent_work](../../../../../functions/src/llm/agent/project_memory/recent_work.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
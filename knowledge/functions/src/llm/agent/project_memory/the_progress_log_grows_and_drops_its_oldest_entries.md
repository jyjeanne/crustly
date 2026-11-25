---
type: Rust Function
title: the_progress_log_grows_and_drops_its_oldest_entries
resource: src/llm/agent/project_memory.rs#L360-L380
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn the_progress_log_grows_and_drops_its_oldest_entries()`

# Calls

- [append_progress](../../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
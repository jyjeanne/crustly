---
type: Rust Function
title: mentioned_paths
resource: src/memory/paths.rs#L41-L65
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/paths_are_found_in_a_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn mentioned_paths(message: &str) -> Vec<String>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [paths_are_found_in_a_message](../../../../functions/src/memory/paths/paths_are_found_in_a_message.md)
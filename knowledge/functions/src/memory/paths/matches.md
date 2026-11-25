---
type: Rust Function
title: matches
resource: src/memory/paths.rs#L142-L154
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/globs_match_paths_and_stop_at_directories
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn matches(globs: &[String], path: &str) -> bool`

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [memories_for_path](../../../../functions/src/memory/MemoryStore/memories_for_path.md)
- [globs_match_paths_and_stop_at_directories](../../../../functions/src/memory/paths/globs_match_paths_and_stop_at_directories.md)
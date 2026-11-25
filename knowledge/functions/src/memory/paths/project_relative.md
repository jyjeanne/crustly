---
type: Rust Function
title: project_relative
resource: src/memory/paths.rs#L72-L93
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/paths/lexical
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/strip_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/as_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/memories_for_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn project_relative(path: &str, working_dir: &Path, root: &Path) -> Option<String>`

# Calls

- [lexical](../../../../functions/src/memory/paths/lexical.md)
- [strip_root](../../../../functions/src/memory/paths/strip_root.md)
- [as_text](../../../../functions/src/memory/paths/as_text.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [memories_for_path](../../../../functions/src/memory/MemoryStore/memories_for_path.md)
---
type: Rust Function
title: validate_glob
resource: src/memory/paths.rs#L19-L37
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/valid_relative_globs_are_normalised
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/globs_that_leave_the_project_are_refused
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_glob(glob: &str) -> Result<String, String>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [add_paths](../../../../functions/src/memory/MemoryStore/add_paths.md)
- [valid_relative_globs_are_normalised](../../../../functions/src/memory/paths/valid_relative_globs_are_normalised.md)
- [globs_that_leave_the_project_are_refused](../../../../functions/src/memory/paths/globs_that_leave_the_project_are_refused.md)
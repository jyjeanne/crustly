---
type: Rust Method
title: bundle_is_older_than_sources
resource: src/memory/code_knowledge.rs#L599-L629
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/newest_mtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn bundle_is_older_than_sources(&self, root: &Path) -> bool`

# Calls

- [newest_mtime](../../../../../functions/src/memory/code_knowledge/newest_mtime.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
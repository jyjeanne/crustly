---
type: Rust Function
title: strip_root
resource: src/memory/paths.rs#L123-L138
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/paths/project_relative
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_root(full: &str, root: &str) -> Option<String>`

# Called by

- [project_relative](../../../../functions/src/memory/paths/project_relative.md)
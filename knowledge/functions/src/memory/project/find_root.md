---
type: Rust Function
title: find_root
resource: src/memory/project.rs#L50-L85
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn find_root(start: &Path) -> Option<(PathBuf, Option<PathBuf>)>`

# Called by

- [identify](../../../../functions/src/memory/project/identify.md)
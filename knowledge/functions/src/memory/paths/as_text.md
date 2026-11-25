---
type: Rust Function
title: as_text
resource: src/memory/paths.rs#L114-L119
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/normalize_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/paths/project_relative
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn as_text(path: &Path) -> String`

# Calls

- [normalize_root](../../../../functions/src/memory/project/normalize_root.md)

# Called by

- [project_relative](../../../../functions/src/memory/paths/project_relative.md)
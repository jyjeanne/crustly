---
type: Rust Function
title: sha256_hex
resource: src/memory/project.rs#L119-L124
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sha256_hex(bytes: &[u8]) -> String`

# Called by

- [id_for_root](../../../../functions/src/memory/project/id_for_root.md)
- [identify](../../../../functions/src/memory/project/identify.md)
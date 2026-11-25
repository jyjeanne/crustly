---
type: Rust Function
title: normalize_root
resource: src/memory/project.rs#L89-L98
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/paths/as_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn normalize_root(path: &str) -> String`

# Calls

- [next](../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [as_text](../../../../functions/src/memory/paths/as_text.md)
- [id_for_root](../../../../functions/src/memory/project/id_for_root.md)
- [identify](../../../../functions/src/memory/project/identify.md)
- [normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive](../../../../functions/src/memory/project/normalize_root_strips_the_verbatim_prefix_and_lowers_the_drive.md)
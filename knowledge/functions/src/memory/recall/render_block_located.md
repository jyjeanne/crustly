---
type: Rust Function
title: render_block_located
resource: src/memory/recall.rs#L244-L288
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/recall/line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/clean
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/render_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_block_located( selected: &[Memory], recent_work: &[String], locations: &std::collections::HashMap<uuid::Uuid, String>, ) -> Option<String>`

# Calls

- [line](../../../../functions/src/memory/recall/line.md)
- [clean](../../../../functions/src/memory/recall/clean.md)

# Called by

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)
- [render_block](../../../../functions/src/memory/recall/render_block.md)
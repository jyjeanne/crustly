---
type: Rust Function
title: is_identifier
resource: src/memory/code_knowledge.rs#L148-L154
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/parse_code_span
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_identifier(s: &str) -> bool`

# Calls

- [next](../../../../functions/src/tui/events/EventHandler/next.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [parse_code_span](../../../../functions/src/memory/code_knowledge/parse_code_span.md)
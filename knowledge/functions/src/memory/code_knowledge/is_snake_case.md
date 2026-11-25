---
type: Rust Function
title: is_snake_case
resource: src/memory/code_knowledge.rs#L157-L165
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
  - target: functions/src/memory/code_knowledge/identifiers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/parse_code_span
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_snake_case(s: &str) -> bool`

# Calls

- [next](../../../../functions/src/tui/events/EventHandler/next.md)
- [all](../../../../functions/src/memory/MemoryStore/all.md)

# Called by

- [identifiers](../../../../functions/src/memory/code_knowledge/identifiers.md)
- [parse_code_span](../../../../functions/src/memory/code_knowledge/parse_code_span.md)
---
type: Rust Function
title: parse_code_span
resource: src/memory/code_knowledge.rs#L99-L133
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_identifier
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_snake_case
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_camel_case
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/identifiers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_code_span(span: &str) -> Option<Identifier>`

# Calls

- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [next](../../../../functions/src/tui/events/EventHandler/next.md)
- [is_identifier](../../../../functions/src/memory/code_knowledge/is_identifier.md)
- [is_snake_case](../../../../functions/src/memory/code_knowledge/is_snake_case.md)
- [is_camel_case](../../../../functions/src/memory/code_knowledge/is_camel_case.md)

# Called by

- [identifiers](../../../../functions/src/memory/code_knowledge/identifiers.md)
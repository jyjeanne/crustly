---
type: Rust Function
title: identifiers
resource: src/memory/code_knowledge.rs#L72-L95
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/parse_code_span
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_snake_case
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_camel_case
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn identifiers(text: &str) -> Vec<Identifier>`

# Calls

- [parse_code_span](../../../../functions/src/memory/code_knowledge/parse_code_span.md)
- [contains](../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [is_snake_case](../../../../functions/src/memory/code_knowledge/is_snake_case.md)
- [is_camel_case](../../../../functions/src/memory/code_knowledge/is_camel_case.md)

# Called by

- [resolve_text](../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)
- [names](../../../../functions/src/memory/code_knowledge/names.md)
- [real_okf_mcp_answers_in_json](../../../../functions/tests/memory_code_knowledge_test/real_okf_mcp_answers_in_json.md)
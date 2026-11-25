---
type: Rust Method
title: resolve
resource: src/memory/code_knowledge.rs#L430-L463
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/call_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/is_test_concept
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn resolve(&self, ident: &Identifier) -> Option<String>`

# Calls

- [call_json](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/call_json.md)
- [is_test_concept](../../../../../functions/src/memory/code_knowledge/is_test_concept.md)

# Called by

- [resolve_text](../../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve_text.md)
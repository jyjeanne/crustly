---
type: Rust Function
title: is_test_concept
resource: src/memory/code_knowledge.rs#L138-L146
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/resolve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/test_code_is_recognised_by_its_path_or_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_test_concept(id: &str) -> bool`

# Called by

- [resolve](../../../../functions/src/memory/code_knowledge/CodeKnowledge/resolve.md)
- [test_code_is_recognised_by_its_path_or_name](../../../../functions/src/memory/code_knowledge/test_code_is_recognised_by_its_path_or_name.md)
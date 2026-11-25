---
type: Rust Method
title: with_code_knowledge
resource: src/memory/anchors.rs#L43-L55
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_code_knowledge(mut self, detection: Detection) -> Self`

# Called by

- [build_memory_store](../../../../../functions/src/cli/build_memory_store.md)
- [setup](../../../../../functions/tests/memory_code_knowledge_test/setup.md)
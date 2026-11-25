---
type: Rust Method
title: call
resource: tests/memory_code_knowledge_test.rs#L132-L187
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/Scripted/reference
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn call(&mut self, tool: &str, args: Value) -> anyhow::Result<String>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [reference](../../../../../functions/tests/memory_code_knowledge_test/Scripted/reference.md)
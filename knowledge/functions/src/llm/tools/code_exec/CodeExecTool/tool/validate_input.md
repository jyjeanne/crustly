---
type: Rust Method
title: validate_input
resource: src/llm/tools/code_exec.rs#L93-L126
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
---
type: Rust Method
title: validate_input
resource: src/llm/tools/skill.rs#L102-L118
visibility: private
generated:
  by: okf-rs/0.8.0
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
---
type: Rust Method
title: after_tools
resource: src/llm/agent/middleware.rs#L515-L537
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Verification/is_check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn after_tools(&mut self, records: &[ToolCallRecord])`

# Calls

- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [is_check](../../../../../../../functions/src/llm/agent/middleware/Verification/is_check.md)
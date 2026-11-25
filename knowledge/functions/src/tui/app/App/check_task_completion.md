---
type: Rust Method
title: check_task_completion
resource: src/tui/app.rs#L2628-L2670
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/save_plan
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn check_task_completion(&mut self, response_content: &str) -> Result<bool>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [save_plan](../../../../../functions/src/tui/app/App/save_plan.md)

# Called by

- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
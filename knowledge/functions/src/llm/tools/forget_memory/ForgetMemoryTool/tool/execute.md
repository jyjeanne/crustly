---
type: Rust Method
title: execute
resource: src/llm/tools/forget_memory.rs#L136-L162
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/forget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/forget_memory/forgetting_removes_the_memories_and_records_the_actor
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/one_unknown_id_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [forget](../../../../../../../functions/src/memory/MemoryStore/forget.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)

# Called by

- [forgetting_removes_the_memories_and_records_the_actor](../../../../../../../functions/src/llm/tools/forget_memory/forgetting_removes_the_memories_and_records_the_actor.md)
- [one_unknown_id_deletes_nothing](../../../../../../../functions/src/llm/tools/forget_memory/one_unknown_id_deletes_nothing.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
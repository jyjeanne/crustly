---
type: Rust Method
title: new
resource: src/llm/tools/forget_memory.rs#L28-L30
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/the_preview_shows_each_memory_before_anything_is_deleted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/forgetting_removes_the_memories_and_records_the_actor
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/one_unknown_id_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/too_many_ids_are_refused
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(store: Arc<MemoryStore>) -> Self`

# Called by

- [build_tool_registry](../../../../../../functions/src/cli/build_tool_registry.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [the_preview_shows_each_memory_before_anything_is_deleted](../../../../../../functions/src/llm/tools/forget_memory/the_preview_shows_each_memory_before_anything_is_deleted.md)
- [forgetting_removes_the_memories_and_records_the_actor](../../../../../../functions/src/llm/tools/forget_memory/forgetting_removes_the_memories_and_records_the_actor.md)
- [one_unknown_id_deletes_nothing](../../../../../../functions/src/llm/tools/forget_memory/one_unknown_id_deletes_nothing.md)
- [too_many_ids_are_refused](../../../../../../functions/src/llm/tools/forget_memory/too_many_ids_are_refused.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
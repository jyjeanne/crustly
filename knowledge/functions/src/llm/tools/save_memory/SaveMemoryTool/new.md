---
type: Rust Method
title: new
resource: src/llm/tools/save_memory.rs#L38-L40
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(store: Option<Arc<MemoryStore>>) -> Self`

# Called by

- [build_tool_registry](../../../../../../functions/src/cli/build_tool_registry.md)
- [a_refused_global_save_stores_nothing](../../../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [database_save_can_be_global_with_a_category](../../../../../../functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category.md)
- [database_save_refuses_episodes_bad_expiry_and_secrets](../../../../../../functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets.md)
- [database_replace_with_an_unknown_id_is_an_error](../../../../../../functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error.md)
- [only_global_saves_with_a_store_need_approval](../../../../../../functions/src/llm/tools/save_memory/only_global_saves_with_a_store_need_approval.md)
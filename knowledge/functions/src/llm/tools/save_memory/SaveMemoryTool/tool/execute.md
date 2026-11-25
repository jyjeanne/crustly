---
type: Rust Method
title: execute
resource: src/llm/tools/save_memory.rs#L285-L330
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/memory_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/append_fact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/save_memory/execute_creates_memory_file_with_header_and_fact
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/execute_appends_to_existing_memory_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/execute_does_not_duplicate_an_identical_fact
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/execute_blocked_in_read_only_mode
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
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [save_to_store](../../../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [memory_path](../../../../../../../functions/src/llm/tools/save_memory/memory_path.md)
- [append_fact](../../../../../../../functions/src/llm/tools/save_memory/append_fact.md)
- [write](../../../../../../../functions/src/memory/project/write.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)

# Called by

- [execute_creates_memory_file_with_header_and_fact](../../../../../../../functions/src/llm/tools/save_memory/execute_creates_memory_file_with_header_and_fact.md)
- [execute_appends_to_existing_memory_file](../../../../../../../functions/src/llm/tools/save_memory/execute_appends_to_existing_memory_file.md)
- [execute_does_not_duplicate_an_identical_fact](../../../../../../../functions/src/llm/tools/save_memory/execute_does_not_duplicate_an_identical_fact.md)
- [memory_persists_across_different_sessions_in_the_same_directory](../../../../../../../functions/src/llm/tools/save_memory/memory_persists_across_different_sessions_in_the_same_directory.md)
- [execute_blocked_in_read_only_mode](../../../../../../../functions/src/llm/tools/save_memory/execute_blocked_in_read_only_mode.md)
- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [database_save_can_be_global_with_a_category](../../../../../../../functions/src/llm/tools/save_memory/database_save_can_be_global_with_a_category.md)
- [database_save_refuses_episodes_bad_expiry_and_secrets](../../../../../../../functions/src/llm/tools/save_memory/database_save_refuses_episodes_bad_expiry_and_secrets.md)
- [database_replace_with_an_unknown_id_is_an_error](../../../../../../../functions/src/llm/tools/save_memory/database_replace_with_an_unknown_id_is_an_error.md)
---
type: Rust Function
title: import
resource: src/memory/transfer.rs#L116-L172
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/model/MemoryCategory/fromstr/from_str
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/an_unknown_export_version_is_refused
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn import(store: &MemoryStore, file: &ExportFile) -> Result<TransferReport>`

# Calls

- [id_for_root](../../../../functions/src/memory/project/id_for_root.md)
- [register_project](../../../../functions/src/memory/MemoryStore/register_project.md)
- [from_str](../../../../functions/src/memory/model/MemoryCategory/fromstr/from_str.md)
- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [save](../../../../functions/src/memory/MemoryStore/save.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [an_unknown_export_version_is_refused](../../../../functions/tests/memory_transfer_test/an_unknown_export_version_is_refused.md)
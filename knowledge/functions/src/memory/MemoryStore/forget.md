---
type: Rust Method
title: forget
resource: src/memory/mod.rs#L638-L642
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/GatedProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/forget_selected_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn forget(&self, id: Uuid, actor: &str) -> Result<()>`

# Calls

- [delete](../../../../functions/src/db/repository/memory/MemoryRepository/delete.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [complete](../../../../functions/src/llm/agent/service/GatedProvider/provider/complete.md)
- [execute](../../../../functions/src/llm/tools/forget_memory/ForgetMemoryTool/tool/execute.md)
- [forget_command](../../../../functions/src/tui/app/App/forget_command.md)
- [forget_selected_memory](../../../../functions/src/tui/app/App/forget_selected_memory.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)
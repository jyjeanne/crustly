---
type: Rust Function
title: import_from_notes
resource: src/memory/transfer.rs#L277-L329
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/split_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/remove_bullets
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/from_notes_without_a_file_does_nothing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn import_from_notes( store: &MemoryStore, project: &ProjectIdentity, notes: &Path, backup_stamp: &str, ) -> Result<(TransferReport, Option<PathBuf>)>`

# Calls

- [context](../../../../functions/src/services/ServiceManager/context.md)
- [split_notes](../../../../functions/src/memory/transfer/split_notes.md)
- [register_project](../../../../functions/src/memory/MemoryStore/register_project.md)
- [save](../../../../functions/src/memory/MemoryStore/save.md)
- [write](../../../../functions/src/memory/project/write.md)
- [remove_bullets](../../../../functions/src/memory/transfer/remove_bullets.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first](../../../../functions/tests/memory_transfer_test/from_notes_moves_bullets_keeps_rejected_ones_and_backs_up_first.md)
- [from_notes_without_a_file_does_nothing](../../../../functions/tests/memory_transfer_test/from_notes_without_a_file_does_nothing.md)
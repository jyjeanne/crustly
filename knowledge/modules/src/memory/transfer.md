---
type: Rust Module
title: transfer
resource: src/memory/transfer.rs#L1-L393
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-model-memorycategory-memoryscope-memorysource-memorystatus-newmemory-rejectreason-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-project-id-for-root-projectidentity
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-memorystore
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-bail-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-datetime-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-str-fromstr
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ExportedMemory](../../../classes/src/memory/transfer/ExportedMemory.md)
- [ExportFile](../../../classes/src/memory/transfer/ExportFile.md)
- [ExportScope](../../../classes/src/memory/transfer/ExportScope.md)
- [TransferReport](../../../classes/src/memory/transfer/TransferReport.md)
- [export](../../../functions/src/memory/transfer/export.md)
- [import](../../../functions/src/memory/transfer/import.md)
- [record](../../../functions/src/memory/transfer/record.md)
- [reason_text](../../../functions/src/memory/transfer/reason_text.md)
- [NotesSplit](../../../classes/src/memory/transfer/NotesSplit.md)
- [split_notes](../../../functions/src/memory/transfer/split_notes.md)
- [remove_bullets](../../../functions/src/memory/transfer/remove_bullets.md)
- [import_from_notes](../../../functions/src/memory/transfer/import_from_notes.md)
- [backup_stamp](../../../functions/src/memory/transfer/backup_stamp.md)
- [split_finds_only_the_bullets_of_the_memories_section](../../../functions/src/memory/transfer/split_finds_only_the_bullets_of_the_memories_section.md)
- [remove_drops_only_the_named_bullets_and_keeps_the_rest](../../../functions/src/memory/transfer/remove_drops_only_the_named_bullets_and_keeps_the_rest.md)
- [the_heading_goes_when_no_bullet_is_left](../../../functions/src/memory/transfer/the_heading_goes_when_no_bullet_is_left.md)
- [a_file_without_the_section_is_unchanged](../../../functions/src/memory/transfer/a_file_without_the_section_is_unchanged.md)
- [backup_stamps_sort_by_time](../../../functions/src/memory/transfer/backup_stamps_sort_by_time.md)

# Imports

- `super::model::{
    MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, RejectReason, SaveOutcome,
}`
- `super::project::{id_for_root, ProjectIdentity}`
- `super::MemoryStore`
- `anyhow::{bail, Context, Result}`
- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `std::path::{Path, PathBuf}`
- `std::str::FromStr`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)
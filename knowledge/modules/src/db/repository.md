---
type: Rust Module
title: repository
resource: src/db/repository/mod.rs#L1-L38
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-compaction-compactionrecordrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-file-filerepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-memory-memoryrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-message-messagerepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-plan-planrepository-plantaskrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-session-sessionlistoptions-sessionrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-result
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Repository](../../../interfaces/src/db/repository/Repository.md)

# Imports

- `pub use compaction::CompactionRecordRepository`
- `pub use file::FileRepository`
- `pub use memory::MemoryRepository`
- `pub use message::MessageRepository`
- `pub use plan::{PlanRepository, PlanTaskRepository}`
- `pub use session::{SessionListOptions, SessionRepository}`
- `anyhow::Result`

# Member of

- [crustly](../../../packages/crustly.md)
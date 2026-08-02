---
type: Rust Module
title: repository
resource: src/db/repository/mod.rs#L1-L38
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/pub-use-compaction-compactionrecordrepository
  - external/pub-use-file-filerepository
  - external/pub-use-memory-episodicmemoryrepository
  - external/pub-use-message-messagerepository
  - external/pub-use-plan-planrepository-plantaskrepository
  - external/pub-use-session-sessionlistoptions-sessionrepository
  - external/anyhow-result
  member_of:
  - packages/crustly
---

# Contains

- [Repository](../../../interfaces/src/db/repository/Repository.md)

# Imports

- `pub use compaction::CompactionRecordRepository`
- `pub use file::FileRepository`
- `pub use memory::EpisodicMemoryRepository`
- `pub use message::MessageRepository`
- `pub use plan::{PlanRepository, PlanTaskRepository}`
- `pub use session::{SessionListOptions, SessionRepository}`
- `anyhow::Result`

# Member of

- [crustly](../../../packages/crustly.md)
---
type: Rust Module
title: memory
resource: src/db/repository/memory.rs#L1-L254
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/crate-llm-agent-memory-episodicmemory
  - external/crate-llm-provider-types-contentblock-message-role
  - external/anyhow-result
  - external/sqlx-sqlitepool
  - external/uuid-uuid
  - external/super
  - external/crate-llm-agent-context-agentcontext
  member_of:
  - packages/crustly
---

# Contains

- [EpisodicMemoryRepository](../../../../classes/src/db/repository/memory/EpisodicMemoryRepository.md)
- [new](../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/new.md)
- [insert](../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/insert.md)
- [list_recent](../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent.md)
- [inject_into_context](../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/inject_into_context.md)
- [create_test_pool](../../../../functions/src/db/repository/memory/create_test_pool.md)
- [episodic_memory_inject_3_memories_within_budget](../../../../functions/src/db/repository/memory/episodic_memory_inject_3_memories_within_budget.md)
- [list_recent_truncates_multibyte_summary_without_panicking](../../../../functions/src/db/repository/memory/list_recent_truncates_multibyte_summary_without_panicking.md)

# Imports

- `crate::llm::agent::memory::EpisodicMemory`
- `crate::llm::provider::types::{ContentBlock, Message, Role}`
- `anyhow::Result`
- `sqlx::SqlitePool`
- `uuid::Uuid`
- `super::*`
- `crate::llm::agent::context::AgentContext`

# Member of

- [crustly](../../../../packages/crustly.md)
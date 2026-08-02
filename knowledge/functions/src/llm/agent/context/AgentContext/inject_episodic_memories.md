---
type: Rust Method
title: inject_episodic_memories
resource: src/llm/agent/context.rs#L180-L188
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/memory/EpisodicMemoryRepository/inject_into_context
  called_by:
  - functions/src/db/repository/memory/episodic_memory_inject_3_memories_within_budget
---

# Signature

`pub async fn inject_episodic_memories( &mut self, pool: &sqlx::SqlitePool, max_tokens: i32, ) -> anyhow::Result<()>`

# Calls

- [inject_into_context](../../../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/inject_into_context.md)

# Called by

- [episodic_memory_inject_3_memories_within_budget](../../../../../../functions/src/db/repository/memory/episodic_memory_inject_3_memories_within_budget.md)
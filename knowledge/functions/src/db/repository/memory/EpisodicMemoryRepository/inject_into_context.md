---
type: Rust Method
title: inject_into_context
resource: src/db/repository/memory.rs#L99-L130
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/context/AgentContext/inject_episodic_memories
---

# Signature

`pub async fn inject_into_context( &self, ctx: &mut crate::llm::agent::context::AgentContext, max_tokens: i32, ) -> Result<()>`

# Calls

- [list_recent](../../../../../../functions/src/db/repository/memory/EpisodicMemoryRepository/list_recent.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [inject_episodic_memories](../../../../../../functions/src/llm/agent/context/AgentContext/inject_episodic_memories.md)
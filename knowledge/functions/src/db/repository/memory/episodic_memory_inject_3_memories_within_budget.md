---
type: Rust Function
title: episodic_memory_inject_3_memories_within_budget
resource: src/db/repository/memory.rs#L159-L221
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/token_count
  - functions/src/llm/agent/context/AgentContext/inject_episodic_memories
---

# Signature

`async fn episodic_memory_inject_3_memories_within_budget()`

# Calls

- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)
- [inject_episodic_memories](../../../../../functions/src/llm/agent/context/AgentContext/inject_episodic_memories.md)
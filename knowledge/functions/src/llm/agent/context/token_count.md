---
type: Rust Function
title: token_count
resource: src/llm/agent/context.rs#L195-L209
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/db/repository/memory/episodic_memory_inject_3_memories_within_budget
  - functions/src/llm/agent/compaction/compact
  - functions/src/llm/agent/context/AgentContext/estimate_tokens
  - functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file
  - functions/src/llm/agent/context/token_count_prose_reasonable
  - functions/src/services/session/SessionService/end_session_with_summary
---

# Signature

`pub fn token_count(text: &str) -> u32`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [episodic_memory_inject_3_memories_within_budget](../../../../../functions/src/db/repository/memory/episodic_memory_inject_3_memories_within_budget.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
- [estimate_tokens](../../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)
- [token_count_bpe_accuracy_rust_file](../../../../../functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file.md)
- [token_count_prose_reasonable](../../../../../functions/src/llm/agent/context/token_count_prose_reasonable.md)
- [end_session_with_summary](../../../../../functions/src/services/session/SessionService/end_session_with_summary.md)
---
type: Rust Function
title: compaction_atomicity_db_failure_leaves_context_unchanged
resource: src/llm/agent/compaction.rs#L218-L255
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/compaction/compact
---

# Signature

`async fn compaction_atomicity_db_failure_leaves_context_unchanged()`

# Calls

- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
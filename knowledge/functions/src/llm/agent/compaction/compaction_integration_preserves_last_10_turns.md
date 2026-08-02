---
type: Rust Function
title: compaction_integration_preserves_last_10_turns
resource: src/llm/agent/compaction.rs#L258-L316
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/src/llm/agent/compaction/compact
---

# Signature

`async fn compaction_integration_preserves_last_10_turns()`

# Calls

- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
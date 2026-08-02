---
type: Rust Method
title: should_compact
resource: src/llm/agent/context.rs#L159-L162
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/compaction/compaction_fires_at_threshold
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn should_compact(&self) -> bool`

# Called by

- [compaction_fires_at_threshold](../../../../../../functions/src/llm/agent/compaction/compaction_fires_at_threshold.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
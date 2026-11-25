---
type: Rust Method
title: compact_deeper
resource: src/llm/agent/service/run.rs#L435-L452
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn compact_deeper(&mut self, overflow_retries: &mut usize) -> bool`

# Calls

- [make_room](../../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)

# Called by

- [call_model](../../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
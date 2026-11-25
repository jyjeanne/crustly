---
type: Rust Method
title: after_tools
resource: src/llm/agent/middleware.rs#L130-L134
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn after_tools(&mut self, records: &[ToolCallRecord])`

# Called by

- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
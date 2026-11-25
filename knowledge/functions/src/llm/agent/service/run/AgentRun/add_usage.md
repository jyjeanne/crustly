---
type: Rust Method
title: add_usage
resource: src/llm/agent/service/run.rs#L329-L337
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn add_usage(&self, response: &LLMResponse, totals: &mut Totals)`

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
- [finish](../../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)
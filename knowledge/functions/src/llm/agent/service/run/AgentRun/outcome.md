---
type: Rust Method
title: outcome
resource: src/llm/agent/service/run.rs#L303-L327
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn outcome(&self, response: LLMResponse, notice: Option<String>, totals: Totals) -> RunOutcome`

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)
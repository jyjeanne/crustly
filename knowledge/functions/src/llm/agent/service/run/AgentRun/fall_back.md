---
type: Rust Method
title: fall_back
resource: src/llm/agent/service/run.rs#L457-L496
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fall_back(&mut self, overflow: bool, error: &ProviderError, totals: &mut Totals) -> bool`

# Called by

- [call_model](../../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
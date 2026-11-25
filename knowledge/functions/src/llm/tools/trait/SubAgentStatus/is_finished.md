---
type: Rust Method
title: is_finished
resource: src/llm/tools/trait.rs#L42-L44
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/cancel
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_finished(self) -> bool`

# Called by

- [report](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)
- [cancel](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/cancel.md)
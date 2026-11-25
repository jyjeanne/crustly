---
type: Rust Function
title: plan_completion_rejection
resource: src/llm/agent/service.rs#L96-L163
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/plan_completion_gate_decision_matrix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plan_completion_rejection( input: &Value, mutating_evidence: usize, working_directory: &std::path::Path, session_id: Uuid, ) -> Option<String>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [plan_completion_gate_decision_matrix](../../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
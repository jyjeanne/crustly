---
type: Rust Function
title: plan_completion_rejection
resource: src/llm/agent/service.rs#L157-L224
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/plan_completion_gate_decision_matrix
---

# Signature

`fn plan_completion_rejection( input: &Value, mutating_evidence: usize, working_directory: &std::path::Path, session_id: Uuid, ) -> Option<String>`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [plan_completion_gate_decision_matrix](../../../../../functions/src/llm/agent/service/plan_completion_gate_decision_matrix.md)
---
type: Rust Method
title: effective_system_prompt
resource: src/llm/agent/context.rs#L70-L85
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/build_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_system_prompt(&self) -> Option<String>`

# Called by

- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [measure_budget](../../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [build_request](../../../../../../functions/src/llm/agent/service/AgentService/build_request.md)
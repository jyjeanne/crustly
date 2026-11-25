---
type: Rust Method
title: run
resource: src/llm/agent/service/subagents.rs#L434-L436
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn run(&self, task: SubAgentTask, cancel: CancellationToken) -> SubAgentReport`

# Calls

- [run](../../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/run.md)

# Called by

- [an_unknown_profile_fails_with_the_available_ones](../../../../../../../../functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones.md)
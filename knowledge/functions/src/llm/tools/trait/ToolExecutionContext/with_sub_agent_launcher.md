---
type: Rust Method
title: with_sub_agent_launcher
resource: src/llm/tools/trait.rs#L182-L185
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_sub_agent_launcher(mut self, launcher: Arc<dyn SubAgentLauncher>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [run_waits_for_the_answer_and_returns_it](../../../../../../functions/src/llm/tools/agent/run_waits_for_the_answer_and_returns_it.md)
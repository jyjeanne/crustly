---
type: Rust Method
title: with_sub_agent_launcher
resource: src/llm/tools/trait.rs#L106-L109
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn with_sub_agent_launcher(mut self, launcher: Arc<dyn SubAgentLauncher>) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
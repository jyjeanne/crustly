---
type: Rust Function
title: sub_agent_launcher_does_not_auto_approve_tools
resource: src/llm/agent/service.rs#L2247-L2264
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch
---

# Signature

`async fn sub_agent_launcher_does_not_auto_approve_tools()`

# Calls

- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)
- [launch](../../../../../functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch.md)
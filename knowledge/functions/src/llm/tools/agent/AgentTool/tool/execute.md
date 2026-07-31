---
type: Rust Method
title: execute
resource: src/llm/tools/agent.rs#L115-L202
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/tools/agent/slugify
  - functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch
  - functions/src/llm/tools/trait/ToolResult/with_metadata
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [is_empty](../../../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [slugify](../../../../../../../functions/src/llm/tools/agent/slugify.md)
- [launch](../../../../../../../functions/src/llm/agent/service/AgentServiceLauncher/crate-llm-tools-subagentlauncher/launch.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
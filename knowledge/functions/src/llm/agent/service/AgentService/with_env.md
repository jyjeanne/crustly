---
type: Rust Method
title: with_env
resource: src/llm/agent/service.rs#L812-L821
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/system_prompt_with_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn with_env(&self, base: Option<&str>) -> Option<String>`

# Called by

- [system_prompt_with_env](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_with_env.md)
- [system_prompt_for](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
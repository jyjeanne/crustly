---
type: Rust Method
title: is_trusted
resource: src/llm/tools/registry.rs#L67-L74
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/canonical_name
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias
---

# Signature

`pub fn is_trusted(&self, name: &str, input: &serde_json::Value) -> bool`

# Calls

- [canonical_name](../../../../../../functions/src/llm/tools/registry/ToolRegistry/canonical_name.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias](../../../../../../functions/src/llm/tools/registry/is_trusted_evaluates_policy_against_the_canonical_name_not_the_alias.md)
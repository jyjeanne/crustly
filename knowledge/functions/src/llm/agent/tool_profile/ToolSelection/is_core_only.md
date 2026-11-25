---
type: Rust Method
title: is_core_only
resource: src/llm/agent/tool_profile.rs#L106-L108
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/auto_uses_core_tools_below_32k_and_every_tool_above
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/explicit_profiles_ignore_the_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_core_only(&self) -> bool`

# Called by

- [system_prompt_for](../../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [auto_uses_core_tools_below_32k_and_every_tool_above](../../../../../../functions/src/llm/agent/tool_profile/auto_uses_core_tools_below_32k_and_every_tool_above.md)
- [explicit_profiles_ignore_the_window](../../../../../../functions/src/llm/agent/tool_profile/explicit_profiles_ignore_the_window.md)
- [an_allow_list_exposes_exactly_its_tools](../../../../../../functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools.md)
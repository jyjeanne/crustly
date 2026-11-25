---
type: Rust Method
title: resolve
resource: src/llm/agent/tool_profile.rs#L87-L103
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/tool_selection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/core_exposes_core_extra_and_mcp_tools_only
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/mcp_exposure_follows_each_servers_setting
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/auto_uses_core_tools_below_32k_and_every_tool_above
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/explicit_profiles_ignore_the_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn resolve(profile: ToolProfile, extra: &[String], context_window: u32) -> Self`

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [tool_selection](../../../../../../functions/src/llm/agent/service/AgentService/tool_selection.md)
- [core_exposes_core_extra_and_mcp_tools_only](../../../../../../functions/src/llm/agent/tool_profile/core_exposes_core_extra_and_mcp_tools_only.md)
- [mcp_exposure_follows_each_servers_setting](../../../../../../functions/src/llm/agent/tool_profile/mcp_exposure_follows_each_servers_setting.md)
- [auto_uses_core_tools_below_32k_and_every_tool_above](../../../../../../functions/src/llm/agent/tool_profile/auto_uses_core_tools_below_32k_and_every_tool_above.md)
- [explicit_profiles_ignore_the_window](../../../../../../functions/src/llm/agent/tool_profile/explicit_profiles_ignore_the_window.md)
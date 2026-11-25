---
type: Rust Module
title: tool_profile
resource: src/llm/agent/tool_profile.rs#L1-L218
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-config-mcpexpose-toolprofile
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ToolSelection](../../../../classes/src/llm/agent/tool_profile/ToolSelection.md)
- [full](../../../../functions/src/llm/agent/tool_profile/ToolSelection/full.md)
- [only](../../../../functions/src/llm/agent/tool_profile/ToolSelection/only.md)
- [with_mcp_exposure](../../../../functions/src/llm/agent/tool_profile/ToolSelection/with_mcp_exposure.md)
- [exposes_mcp](../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes_mcp.md)
- [resolve](../../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [is_core_only](../../../../functions/src/llm/agent/tool_profile/ToolSelection/is_core_only.md)
- [exposes](../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [label](../../../../functions/src/llm/agent/tool_profile/ToolSelection/label.md)
- [auto_uses_core_tools_below_32k_and_every_tool_above](../../../../functions/src/llm/agent/tool_profile/auto_uses_core_tools_below_32k_and_every_tool_above.md)
- [explicit_profiles_ignore_the_window](../../../../functions/src/llm/agent/tool_profile/explicit_profiles_ignore_the_window.md)
- [core_exposes_core_extra_and_mcp_tools_only](../../../../functions/src/llm/agent/tool_profile/core_exposes_core_extra_and_mcp_tools_only.md)
- [an_allow_list_exposes_exactly_its_tools](../../../../functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools.md)
- [mcp_exposure_follows_each_servers_setting](../../../../functions/src/llm/agent/tool_profile/mcp_exposure_follows_each_servers_setting.md)
- [full_exposes_everything](../../../../functions/src/llm/agent/tool_profile/full_exposes_everything.md)

# Imports

- `crate::config::{McpExpose, ToolProfile}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)
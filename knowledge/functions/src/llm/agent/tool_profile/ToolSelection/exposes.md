---
type: Rust Method
title: exposes
resource: src/llm/agent/tool_profile.rs#L79-L87
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/core_exposes_core_extra_and_mcp_tools_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/full_exposes_everything
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn exposes(&self, name: &str) -> bool`

# Calls

- [contains](../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [exposed_tool_definitions](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)
- [execute_tool_round](../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [core_exposes_core_extra_and_mcp_tools_only](../../../../../../functions/src/llm/agent/tool_profile/core_exposes_core_extra_and_mcp_tools_only.md)
- [an_allow_list_exposes_exactly_its_tools](../../../../../../functions/src/llm/agent/tool_profile/an_allow_list_exposes_exactly_its_tools.md)
- [full_exposes_everything](../../../../../../functions/src/llm/agent/tool_profile/full_exposes_everything.md)
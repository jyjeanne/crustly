---
type: Rust Method
title: get_tool_definitions
resource: src/llm/tools/registry.rs#L144-L153
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_tool_definitions(&self) -> Vec<crate::llm::provider::Tool>`

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [exposed_tool_definitions](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_definitions.md)
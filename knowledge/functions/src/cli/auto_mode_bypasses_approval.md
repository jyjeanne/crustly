---
type: Rust Function
title: auto_mode_bypasses_approval
resource: src/cli/mod.rs#L1981-L2002
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/plan/PlanModeState/is_high_risk_tool
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/build_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/auto_mode_bypasses_approval_interactive_never_bypasses
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/global_memory_saves_prompt_in_autoplan_but_not_fullauto
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/autoplan_prompts_for_unlisted_tools_with_risky_capabilities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/auto_mode_bypasses_approval_fullauto_bypasses_everything
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn auto_mode_bypasses_approval( mode: &crate::config::PlanExecMode, tool_name: &str, capabilities: &[String], ) -> bool`

# Calls

- [is_high_risk_tool](../../../functions/src/plan/PlanModeState/is_high_risk_tool.md)

# Called by

- [build_approval_callback](../../../functions/src/cli/build_approval_callback.md)
- [auto_mode_bypasses_approval_interactive_never_bypasses](../../../functions/src/cli/auto_mode_bypasses_approval_interactive_never_bypasses.md)
- [auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only](../../../functions/src/cli/auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only.md)
- [autoplan_prompts_for_the_registered_code_and_shell_tool_names](../../../functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names.md)
- [global_memory_saves_prompt_in_autoplan_but_not_fullauto](../../../functions/src/cli/global_memory_saves_prompt_in_autoplan_but_not_fullauto.md)
- [autoplan_prompts_for_unlisted_tools_with_risky_capabilities](../../../functions/src/cli/autoplan_prompts_for_unlisted_tools_with_risky_capabilities.md)
- [auto_mode_bypasses_approval_fullauto_bypasses_everything](../../../functions/src/cli/auto_mode_bypasses_approval_fullauto_bypasses_everything.md)
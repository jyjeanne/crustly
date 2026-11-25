---
type: Rust Method
title: measure
resource: src/llm/agent/budget.rs#L51-L66
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/ContextBudget/from_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/tool_schema_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/core_profile_fixed_overhead_fits_a_small_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn measure( window: u32, system_prompt: Option<&str>, tools: &[Tool], history_tokens: usize, calibration: f64, ) -> Self`

# Calls

- [from_counts](../../../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)
- [tool_schema_tokens](../../../../../../functions/src/llm/agent/budget/tool_schema_tokens.md)

# Called by

- [core_profile_fixed_overhead_fits_a_small_window](../../../../../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)
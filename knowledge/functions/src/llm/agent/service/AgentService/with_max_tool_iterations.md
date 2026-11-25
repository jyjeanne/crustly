---
type: Rust Method
title: with_max_tool_iterations
resource: src/llm/agent/service.rs#L744-L747
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_max_tool_iterations(mut self, max: usize) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [round_limit_ends_with_an_answer_instead_of_an_error](../../../../../../functions/src/llm/agent/service/round_limit_ends_with_an_answer_instead_of_an_error.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
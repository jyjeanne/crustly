---
type: Rust Method
title: with_progress_file
resource: src/llm/agent/service.rs#L698-L701
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_work_is_remembered_across_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_progress_file(mut self, enabled: bool) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [tool_work_is_remembered_across_requests](../../../../../../functions/src/llm/agent/service/tool_work_is_remembered_across_requests.md)
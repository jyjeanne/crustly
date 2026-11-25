---
type: Rust Method
title: with_project_notes
resource: src/llm/agent/service.rs#L687-L690
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_project_notes(mut self, enabled: bool) -> Self`

# Called by

- [configure_agent_loop](../../../../../../functions/src/cli/configure_agent_loop.md)
- [project_notes_are_added_to_the_system_prompt_when_enabled](../../../../../../functions/src/llm/agent/service/project_notes_are_added_to_the_system_prompt_when_enabled.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
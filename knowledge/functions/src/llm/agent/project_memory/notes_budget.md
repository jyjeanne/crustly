---
type: Rust Function
title: notes_budget
resource: src/llm/agent/project_memory.rs#L36-L38
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/project_memory/notes_budget_is_an_eighth_of_the_window_up_to_2000
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn notes_budget(window: u32) -> u32`

# Called by

- [notes_budget_is_an_eighth_of_the_window_up_to_2000](../../../../../functions/src/llm/agent/project_memory/notes_budget_is_an_eighth_of_the_window_up_to_2000.md)
- [system_prompt_for](../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
---
type: Rust Function
title: load_project_notes
resource: src/llm/agent/project_memory.rs#L42-L82
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/take_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/notes_come_from_the_files_present_in_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_point_at_the_progress_log_when_there_is_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/system_prompt_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_project_notes(dir: &Path, max_tokens: u32) -> Option<String>`

# Calls

- [take_tokens](../../../../../functions/src/llm/agent/project_memory/take_tokens.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [notes_come_from_the_files_present_in_order](../../../../../functions/src/llm/agent/project_memory/notes_come_from_the_files_present_in_order.md)
- [notes_are_truncated_to_the_budget_at_line_ends](../../../../../functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends.md)
- [notes_point_at_the_progress_log_when_there_is_one](../../../../../functions/src/llm/agent/project_memory/notes_point_at_the_progress_log_when_there_is_one.md)
- [system_prompt_for](../../../../../functions/src/llm/agent/service/AgentService/system_prompt_for.md)
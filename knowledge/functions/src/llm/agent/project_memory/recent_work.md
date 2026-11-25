---
type: Rust Function
title: recent_work
resource: src/llm/agent/project_memory.rs#L221-L261
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_is_empty_without_a_log
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn recent_work(dir: &Path, entries: usize, max_tokens: u32) -> Vec<String>`

# Calls

- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [recent_work_gives_the_last_entries_oldest_first](../../../../../functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first.md)
- [recent_work_drops_the_oldest_entries_first_past_the_cap](../../../../../functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap.md)
- [recent_work_is_empty_without_a_log](../../../../../functions/src/llm/agent/project_memory/recent_work_is_empty_without_a_log.md)
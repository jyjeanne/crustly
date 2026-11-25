---
type: Rust Function
title: log_entry
resource: src/llm/agent/project_memory.rs#L382-L392
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn log_entry(dir: &Path, request: &str, result: &str)`

# Calls

- [append_progress](../../../../../functions/src/llm/agent/project_memory/append_progress.md)

# Called by

- [recent_work_gives_the_last_entries_oldest_first](../../../../../functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first.md)
- [recent_work_drops_the_oldest_entries_first_past_the_cap](../../../../../functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap.md)
---
type: Rust Function
title: cleanup_old_logs
resource: src/logging.rs#L266-L296
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn cleanup_old_logs(max_age_days: u64) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)

# Called by

- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
- [main](../../../functions/src/main.md)
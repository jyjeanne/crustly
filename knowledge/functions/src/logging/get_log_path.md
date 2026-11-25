---
type: Rust Function
title: get_log_path
resource: src/logging.rs#L242-L263
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
---

# Signature

`pub fn get_log_path() -> Option<PathBuf>`

# Calls

- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)

# Called by

- [cmd_logs](../../../functions/src/cli/cmd_logs.md)
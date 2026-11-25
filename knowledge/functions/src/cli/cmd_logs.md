---
type: Rust Function
title: cmd_logs
resource: src/cli/mod.rs#L3111-L3273
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/get_log_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/cleanup_old_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_logs(operation: LogCommands) -> Result<()>`

# Calls

- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [get_log_path](../../../functions/src/logging/get_log_path.md)
- [open](../../../functions/tests/memory_acceptance_test/open.md)
- [cleanup_old_logs](../../../functions/src/logging/cleanup_old_logs.md)
- [context](../../../functions/src/services/ServiceManager/context.md)
- [display](../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [run](../../../functions/src/cli/run.md)
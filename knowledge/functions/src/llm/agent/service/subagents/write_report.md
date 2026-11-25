---
type: Rust Function
title: write_report
resource: src/llm/agent/service/subagents.rs#L376-L396
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/agents_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_report(template: &ChildTemplate, task: &SubAgentTask, report: &SubAgentReport)`

# Calls

- [agents_dir](../../../../../../functions/src/llm/agent/service/subagents/agents_dir.md)
- [write](../../../../../../functions/src/memory/project/write.md)

# Called by

- [execute](../../../../../../functions/src/llm/agent/service/subagents/execute.md)
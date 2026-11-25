---
type: Rust Function
title: write_report
resource: src/llm/agent/service/subagents.rs#L378-L398
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/agents_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
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
- [exists](../../../../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)

# Called by

- [execute](../../../../../../functions/src/llm/agent/service/subagents/execute.md)
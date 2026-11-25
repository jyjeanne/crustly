---
type: Rust Function
title: write_answer
resource: src/llm/agent/service/subagents.rs#L399-L409
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/agents_dir
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn write_answer(template: &ChildTemplate, task: &SubAgentTask, answer: &str)`

# Calls

- [agents_dir](../../../../../../functions/src/llm/agent/service/subagents/agents_dir.md)

# Called by

- [execute](../../../../../../functions/src/llm/agent/service/subagents/execute.md)
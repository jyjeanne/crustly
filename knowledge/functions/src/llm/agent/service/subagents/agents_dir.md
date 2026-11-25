---
type: Rust Function
title: agents_dir
resource: src/llm/agent/service/subagents.rs#L372-L374
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/write_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/write_answer
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn agents_dir(template: &ChildTemplate) -> PathBuf`

# Called by

- [write_report](../../../../../../functions/src/llm/agent/service/subagents/write_report.md)
- [write_answer](../../../../../../functions/src/llm/agent/service/subagents/write_answer.md)
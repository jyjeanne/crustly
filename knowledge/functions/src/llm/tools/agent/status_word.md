---
type: Rust Function
title: status_word
resource: src/llm/tools/agent.rs#L236-L244
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/report_result
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn status_word(status: SubAgentStatus) -> &'static str`

# Called by

- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
- [report_result](../../../../../functions/src/llm/tools/agent/report_result.md)
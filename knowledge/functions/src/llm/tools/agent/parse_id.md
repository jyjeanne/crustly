---
type: Rust Function
title: parse_id
resource: src/llm/tools/agent.rs#L220-L224
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/agent/AgentTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/AgentTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_id(id: Option<&str>) -> Result<Uuid>`

# Called by

- [validate_input](../../../../../functions/src/llm/tools/agent/AgentTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/agent/AgentTool/tool/execute.md)
---
type: Rust Function
title: is_blank
resource: src/llm/tools/agent.rs#L281-L283
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/agent/AgentTool/tool/validate_input
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_blank(value: &Option<String>) -> bool`

# Called by

- [validate_input](../../../../../functions/src/llm/tools/agent/AgentTool/tool/validate_input.md)
---
type: Rust Method
title: validate_input
resource: src/llm/tools/agent.rs#L106-L130
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/agent/action_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/is_blank
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/agent/parse_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn validate_input(&self, input: &Value) -> Result<()>`

# Calls

- [action_of](../../../../../../../functions/src/llm/tools/agent/action_of.md)
- [is_blank](../../../../../../../functions/src/llm/tools/agent/is_blank.md)
- [parse_id](../../../../../../../functions/src/llm/tools/agent/parse_id.md)
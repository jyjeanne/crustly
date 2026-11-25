---
type: Rust Function
title: is_parallelizable
resource: src/llm/agent/service.rs#L166-L179
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/loop_detection_recovery_message_logic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_parallelizable(tool_name: &str) -> bool`

# Called by

- [loop_detection_recovery_message_logic](../../../../../functions/src/llm/agent/service/loop_detection_recovery_message_logic.md)
- [execute_tool_round](../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
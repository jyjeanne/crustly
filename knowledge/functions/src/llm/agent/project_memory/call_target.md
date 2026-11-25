---
type: Rust Function
title: call_target
resource: src/llm/agent/project_memory.rs#L134-L154
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/summarize_tool_calls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn call_target(input: &Value) -> Option<String>`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)

# Called by

- [summarize_tool_calls](../../../../../functions/src/llm/agent/project_memory/summarize_tool_calls.md)
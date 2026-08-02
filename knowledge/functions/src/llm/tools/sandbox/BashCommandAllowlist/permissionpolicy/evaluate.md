---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L249-L286
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/find_active_shell_operator
  - functions/src/tui/events/EventHandler/next
---

# Signature

`fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [find_active_shell_operator](../../../../../../../functions/src/llm/tools/sandbox/find_active_shell_operator.md)
- [next](../../../../../../../functions/src/tui/events/EventHandler/next.md)
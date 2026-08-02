---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L362-L372
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/PolicyDecision/is_permitted
---

# Signature

`fn evaluate(&self, tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [is_permitted](../../../../../../../functions/src/llm/tools/sandbox/PolicyDecision/is_permitted.md)
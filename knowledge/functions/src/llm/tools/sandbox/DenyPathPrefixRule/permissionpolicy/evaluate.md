---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L104-L119
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/sandbox/normalize_path
---

# Signature

`fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [normalize_path](../../../../../../../functions/src/llm/tools/sandbox/normalize_path.md)
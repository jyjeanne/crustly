---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L395-L397
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/tools/sandbox/allow_all_never_confers_trust
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn evaluate(&self, _: &str, _: &Value) -> PolicyDecision`

# Called by

- [allow_all_never_confers_trust](../../../../../../../functions/src/llm/tools/sandbox/allow_all_never_confers_trust.md)
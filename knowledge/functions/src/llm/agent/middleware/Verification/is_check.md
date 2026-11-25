---
type: Rust Method
title: is_check
resource: src/llm/agent/middleware.rs#L508-L511
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/Verification/roundmiddleware/after_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_check(&self, command: &str) -> bool`

# Called by

- [after_tools](../../../../../../functions/src/llm/agent/middleware/Verification/roundmiddleware/after_tools.md)
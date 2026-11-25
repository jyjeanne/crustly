---
type: Rust Method
title: new
resource: tests/error_scenarios_test.rs#L38-L40
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(error_type: ErrorType) -> Self`

# Called by

- [create_error_agent](../../../../functions/tests/error_scenarios_test/create_error_agent.md)
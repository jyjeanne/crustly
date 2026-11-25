---
type: Rust Method
title: single_response
resource: tests/integration_test.rs#L37-L39
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/integration_test/MockProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn single_response(response: String) -> Self`

# Calls

- [new](../../../../functions/tests/integration_test/MockProvider/new.md)

# Called by

- [test_end_to_end_system_prompt](../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
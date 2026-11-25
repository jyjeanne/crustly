---
type: Rust Method
title: new
resource: tests/integration_test.rs#L30-L35
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/integration_test/MockProvider/single_response
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(responses: Vec<String>) -> Self`

# Called by

- [single_response](../../../../functions/tests/integration_test/MockProvider/single_response.md)
- [create_test_agent](../../../../functions/tests/integration_test/create_test_agent.md)
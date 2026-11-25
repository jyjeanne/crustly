---
type: Rust Method
title: new
resource: src/llm/provider/factory.rs#L36-L39
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/test_failover_sends_the_secondary_its_own_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(chain: Vec<Arc<dyn Provider>>) -> Self`

# Called by

- [test_failover_on_rate_limit_tries_next_provider](../../../../../../functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider.md)
- [test_failover_sends_the_secondary_its_own_model](../../../../../../functions/src/llm/provider/factory/test_failover_sends_the_secondary_its_own_model.md)
- [test_failover_all_fail_returns_last_error](../../../../../../functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error.md)
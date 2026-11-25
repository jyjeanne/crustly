---
type: Rust Method
title: complete
resource: src/llm/provider/factory.rs#L68-L89
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/FailoverProvider/request_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/FailoverProvider/is_failover_error
    resolved_by: tree-sitter
    confidence: exact
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

`async fn complete( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::types::LLMResponse>`

# Calls

- [request_for](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/request_for.md)
- [is_failover_error](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/is_failover_error.md)

# Called by

- [test_failover_on_rate_limit_tries_next_provider](../../../../../../../functions/src/llm/provider/factory/test_failover_on_rate_limit_tries_next_provider.md)
- [test_failover_sends_the_secondary_its_own_model](../../../../../../../functions/src/llm/provider/factory/test_failover_sends_the_secondary_its_own_model.md)
- [test_failover_all_fail_returns_last_error](../../../../../../../functions/src/llm/provider/factory/test_failover_all_fail_returns_last_error.md)
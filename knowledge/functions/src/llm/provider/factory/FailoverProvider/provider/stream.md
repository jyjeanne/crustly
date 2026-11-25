---
type: Rust Method
title: stream
resource: src/llm/provider/factory.rs#L91-L112
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/factory/FailoverProvider/request_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/FailoverProvider/is_failover_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn stream( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::r#trait::ProviderStream>`

# Calls

- [request_for](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/request_for.md)
- [is_failover_error](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/is_failover_error.md)
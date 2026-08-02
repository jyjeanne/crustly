---
type: Rust Method
title: stream
resource: src/llm/provider/factory.rs#L71-L92
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/factory/FailoverProvider/is_failover_error
---

# Signature

`async fn stream( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::r#trait::ProviderStream>`

# Calls

- [is_failover_error](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/is_failover_error.md)
---
type: Rust Method
title: complete
resource: src/llm/provider/factory.rs#L48-L69
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/factory/FailoverProvider/is_failover_error
---

# Signature

`async fn complete( &self, request: super::types::LLMRequest, ) -> super::error::Result<super::types::LLMResponse>`

# Calls

- [is_failover_error](../../../../../../../functions/src/llm/provider/factory/FailoverProvider/is_failover_error.md)
---
type: Rust Method
title: is_failover_error
resource: src/llm/provider/factory.rs#L36-L43
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/provider/factory/FailoverProvider/provider/complete
  - functions/src/llm/provider/factory/FailoverProvider/provider/stream
---

# Signature

`fn is_failover_error(err: &ProviderError) -> bool`

# Called by

- [complete](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/factory/FailoverProvider/provider/stream.md)
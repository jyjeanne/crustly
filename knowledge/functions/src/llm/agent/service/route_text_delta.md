---
type: Rust Function
title: route_text_delta
resource: src/llm/agent/service.rs#L318-L363
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/agent/service/drain_stream_to_response
---

# Signature

`fn route_text_delta( input: &str, in_think: &mut bool, text_buf: &mut String, thinking_buf: &mut String, chunk_tx: Option<&mpsc::UnboundedSender<String>>, )`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
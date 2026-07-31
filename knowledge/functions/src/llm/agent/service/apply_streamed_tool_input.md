---
type: Rust Function
title: apply_streamed_tool_input
resource: src/llm/agent/service.rs#L373-L398
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/llm/agent/service/drain_stream_to_response
---

# Signature

`fn apply_streamed_tool_input(block: ContentBlock, json_buf: &str) -> ContentBlock`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [from_str](../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [drain_stream_to_response](../../../../../functions/src/llm/agent/service/drain_stream_to_response.md)
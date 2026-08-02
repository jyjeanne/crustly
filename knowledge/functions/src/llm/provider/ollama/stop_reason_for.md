---
type: Rust Function
title: stop_reason_for
resource: src/llm/provider/ollama.rs#L794-L800
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
---

# Signature

`fn stop_reason_for(tool_calls: &[(String, serde_json::Value)]) -> StopReason`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
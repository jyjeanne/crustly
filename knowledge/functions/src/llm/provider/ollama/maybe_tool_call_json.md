---
type: Rust Function
title: maybe_tool_call_json
resource: src/llm/provider/ollama.rs#L807-L810
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/provider/ollama/OllamaProvider/provider/stream
---

# Signature

`fn maybe_tool_call_json(text: &str) -> bool`

# Calls

- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [stream](../../../../../functions/src/llm/provider/ollama/OllamaProvider/provider/stream.md)
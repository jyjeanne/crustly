---
type: Rust Function
title: parse_keep_alive
resource: src/llm/provider/ollama.rs#L958-L976
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/provider/ollama/ModelOverrides/from_config
  - functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive
---

# Signature

`fn parse_keep_alive(s: &str) -> Option<KeepAlive>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [with_keep_alive](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive.md)
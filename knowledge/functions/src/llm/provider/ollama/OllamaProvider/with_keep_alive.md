---
type: Rust Method
title: with_keep_alive
resource: src/llm/provider/ollama.rs#L254-L260
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/parse_keep_alive
  called_by:
  - functions/src/llm/provider/factory/ollama_provider_from_config
---

# Signature

`pub fn with_keep_alive(mut self, keep_alive: &str) -> Self`

# Calls

- [parse_keep_alive](../../../../../../functions/src/llm/provider/ollama/parse_keep_alive.md)

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
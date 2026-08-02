---
type: Rust Method
title: with_think
resource: src/llm/provider/ollama.rs#L204-L207
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama/parse_think
  called_by:
  - functions/src/llm/provider/factory/ollama_provider_from_config
  - functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
  - functions/src/llm/provider/ollama/invalid_think_value_is_ignored
---

# Signature

`pub fn with_think(mut self, think: &str) -> Self`

# Calls

- [parse_think](../../../../../../functions/src/llm/provider/ollama/parse_think.md)

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [invalid_think_value_is_ignored](../../../../../../functions/src/llm/provider/ollama/invalid_think_value_is_ignored.md)
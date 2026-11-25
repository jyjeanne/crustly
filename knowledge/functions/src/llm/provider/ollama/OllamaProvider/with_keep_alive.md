---
type: Rust Method
title: with_keep_alive
resource: src/llm/provider/ollama.rs#L254-L260
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama/parse_keep_alive
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/ollama_provider_from_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_keep_alive(mut self, keep_alive: &str) -> Self`

# Calls

- [parse_keep_alive](../../../../../../functions/src/llm/provider/ollama/parse_keep_alive.md)

# Called by

- [ollama_provider_from_config](../../../../../../functions/src/llm/provider/factory/ollama_provider_from_config.md)
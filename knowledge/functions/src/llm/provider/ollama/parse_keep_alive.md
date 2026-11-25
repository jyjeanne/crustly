---
type: Rust Function
title: parse_keep_alive
resource: src/llm/provider/ollama.rs#L895-L913
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/ModelOverrides/from_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama/test_parse_keep_alive
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_keep_alive(s: &str) -> Option<KeepAlive>`

# Called by

- [from_config](../../../../../functions/src/llm/provider/ollama/ModelOverrides/from_config.md)
- [with_keep_alive](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_keep_alive.md)
- [test_parse_keep_alive](../../../../../functions/src/llm/provider/ollama/test_parse_keep_alive.md)
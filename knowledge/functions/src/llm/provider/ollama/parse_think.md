---
type: Rust Function
title: parse_think
resource: src/llm/provider/ollama.rs#L113-L128
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/ollama/OllamaProvider/with_think
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_think(s: &str) -> Option<ThinkType>`

# Called by

- [with_think](../../../../../functions/src/llm/provider/ollama/OllamaProvider/with_think.md)
---
type: Rust Method
title: set_ollama_config
resource: src/tui/app.rs#L888-L890
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_ollama_config(&mut self, config: crate::config::OllamaProviderConfig)`

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
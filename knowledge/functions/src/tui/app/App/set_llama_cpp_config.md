---
type: Rust Method
title: set_llama_cpp_config
resource: src/tui/app.rs#L920-L925
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_llama_cpp_config(&mut self, config: crate::config::LlamaCppProviderConfig)`

# Calls

- [provider_name](../../../../../functions/src/tui/app/App/provider_name.md)

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
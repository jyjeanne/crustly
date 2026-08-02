---
type: Rust Function
title: ollama_host
resource: src/cli/mod.rs#L1126-L1133
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_ollama
---

# Signature

`fn ollama_host(config: &crate::config::Config) -> String`

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_ollama](../../../functions/src/cli/cmd_ollama.md)
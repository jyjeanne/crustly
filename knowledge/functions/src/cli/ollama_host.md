---
type: Rust Function
title: ollama_host
resource: src/cli/mod.rs#L2460-L2467
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/test_ollama_host_defaults_when_unconfigured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/test_ollama_host_uses_configured_value
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ollama_host(config: &crate::config::Config) -> String`

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_ollama](../../../functions/src/cli/cmd_ollama.md)
- [test_ollama_host_defaults_when_unconfigured](../../../functions/src/cli/test_ollama_host_defaults_when_unconfigured.md)
- [test_ollama_host_uses_configured_value](../../../functions/src/cli/test_ollama_host_uses_configured_value.md)
---
type: Rust Function
title: test_ollama_host_uses_configured_value
resource: src/cli/mod.rs#L3292-L3300
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/ollama_host
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_ollama_host_uses_configured_value()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [ollama_host](../../../functions/src/cli/ollama_host.md)
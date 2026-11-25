---
type: Rust Method
title: default
resource: src/config/mod.rs#L1292-L1305
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_ollama_host
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/test_ollama_provider_config_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/ollama_download/switch_built_provider_applies_per_model_num_ctx_from_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_ollama_host](../../../../../functions/src/config/default_ollama_host.md)

# Called by

- [test_ollama_provider_config_default](../../../../../functions/src/config/test_ollama_provider_config_default.md)
- [switch_built_provider_applies_per_model_num_ctx_from_config](../../../../../functions/src/tui/ollama_download/switch_built_provider_applies_per_model_num_ctx_from_config.md)
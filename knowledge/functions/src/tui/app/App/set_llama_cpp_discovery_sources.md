---
type: Rust Method
title: set_llama_cpp_discovery_sources
resource: src/tui/app.rs#L906-L913
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_llama_cpp_discovery_sources( &mut self, extra_model_paths: Vec<std::path::PathBuf>, ollama_models_dir: Option<std::path::PathBuf>, )`

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)
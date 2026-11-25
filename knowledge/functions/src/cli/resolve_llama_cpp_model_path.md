---
type: Rust Function
title: resolve_llama_cpp_model_path
resource: src/cli/mod.rs#L2402-L2409
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_llama_cpp_model_path(models_dir: &std::path::Path, model: &str) -> std::path::PathBuf`

# Called by

- [cmd_llama_cpp](../../../functions/src/cli/cmd_llama_cpp.md)
- [resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir](../../../functions/src/cli/resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir.md)
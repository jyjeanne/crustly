---
type: Rust Function
title: walk_ollama_manifest_files
resource: src/llm/provider/llama_cpp_models.rs#L387-L414
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn walk_ollama_manifest_files(manifests_root: &Path) -> Vec<PathBuf>`

# Called by

- [list_ollama_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)
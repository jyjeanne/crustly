---
type: Rust Function
title: list_local
resource: src/tui/llama_cpp_download.rs#L169-L227
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn list_local( models_dir: PathBuf, extra_model_paths: Vec<PathBuf>, ollama_models_dir: Option<PathBuf>, ) -> Vec<LlamaCppModelSummary>`

# Calls

- [list_all_local_models](../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
---
type: Rust Function
title: list_local
resource: src/tui/llama_cpp_download.rs#L75-L87
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_models/list_local_models
---

# Signature

`pub async fn list_local(models_dir: PathBuf) -> Vec<LlamaCppModelSummary>`

# Calls

- [list_local_models](../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
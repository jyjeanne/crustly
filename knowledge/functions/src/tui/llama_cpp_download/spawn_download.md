---
type: Rust Function
title: spawn_download
resource: src/tui/llama_cpp_download.rs#L163-L209
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp_models/resolve_download_source
  - functions/src/llm/provider/llama_cpp_models/download_model
---

# Signature

`pub async fn spawn_download( source: String, models_dir: PathBuf, event_sender: UnboundedSender<TuiEvent>, ) -> JoinHandle<()>`

# Calls

- [resolve_download_source](../../../../functions/src/llm/provider/llama_cpp_models/resolve_download_source.md)
- [download_model](../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
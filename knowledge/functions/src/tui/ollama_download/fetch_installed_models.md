---
type: Rust Function
title: fetch_installed_models
resource: src/tui/ollama_download.rs#L79-L84
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/list_models
---

# Signature

`pub async fn fetch_installed_models(host: String) -> Vec<String>`

# Calls

- [list_models](../../../../functions/src/llm/provider/ollama_models/list_models.md)
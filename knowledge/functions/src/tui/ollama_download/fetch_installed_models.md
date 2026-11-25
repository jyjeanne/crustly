---
type: Rust Function
title: fetch_installed_models
resource: src/tui/ollama_download.rs#L79-L84
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/list_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn fetch_installed_models(host: String) -> Vec<String>`

# Calls

- [list_models](../../../../functions/src/llm/provider/ollama_models/list_models.md)
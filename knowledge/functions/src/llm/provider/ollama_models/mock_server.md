---
type: Rust Function
title: mock_server
resource: src/llm/provider/ollama_models.rs#L237-L273
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/ollama_models/list_models_parses_tags_response
  - functions/src/llm/provider/ollama_models/show_model_parses_minimal_response
  - functions/src/llm/provider/ollama_models/delete_model_succeeds_on_2xx
  - functions/src/llm/provider/ollama_models/pull_model_forwards_progress_and_completes
  - functions/src/llm/provider/ollama_models/generate_embeddings_parses_response
---

# Signature

`async fn mock_server(body: String) -> String`

# Called by

- [list_models_parses_tags_response](../../../../../functions/src/llm/provider/ollama_models/list_models_parses_tags_response.md)
- [show_model_parses_minimal_response](../../../../../functions/src/llm/provider/ollama_models/show_model_parses_minimal_response.md)
- [delete_model_succeeds_on_2xx](../../../../../functions/src/llm/provider/ollama_models/delete_model_succeeds_on_2xx.md)
- [pull_model_forwards_progress_and_completes](../../../../../functions/src/llm/provider/ollama_models/pull_model_forwards_progress_and_completes.md)
- [generate_embeddings_parses_response](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings_parses_response.md)
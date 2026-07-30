---
type: Rust Function
title: delete_model_succeeds_on_2xx
resource: src/llm/provider/ollama_models.rs#L305-L310
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/mock_server
  - functions/src/llm/provider/ollama_models/delete_model
---

# Signature

`async fn delete_model_succeeds_on_2xx()`

# Calls

- [mock_server](../../../../../functions/src/llm/provider/ollama_models/mock_server.md)
- [delete_model](../../../../../functions/src/llm/provider/ollama_models/delete_model.md)
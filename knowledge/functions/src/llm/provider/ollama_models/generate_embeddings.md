---
type: Rust Function
title: generate_embeddings
resource: src/llm/provider/ollama_models.rs#L148-L163
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/tui/error/ErrorInfo/with_context
  called_by:
  - functions/src/cli/cmd_ollama
  - functions/src/llm/provider/ollama_models/generate_embeddings_parses_response
---

# Signature

`pub async fn generate_embeddings( host: &str, model_name: &str, input: Vec<String>, ) -> Result<Vec<Vec<f32>>>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [generate_embeddings_parses_response](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings_parses_response.md)
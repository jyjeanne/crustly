---
type: Rust Function
title: generate_embeddings
resource: src/llm/provider/ollama_models.rs#L148-L163
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/client_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/generate_embeddings_parses_response
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn generate_embeddings( host: &str, model_name: &str, input: Vec<String>, ) -> Result<Vec<Vec<f32>>>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)

# Called by

- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [generate_embeddings_parses_response](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings_parses_response.md)
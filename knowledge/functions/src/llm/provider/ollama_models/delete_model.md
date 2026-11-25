---
type: Rust Function
title: delete_model
resource: src/llm/provider/ollama_models.rs#L102-L109
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/ollama_models/client_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn delete_model(host: &str, model_name: &str) -> Result<()>`

# Calls

- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
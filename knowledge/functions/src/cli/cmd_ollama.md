---
type: Rust Function
title: cmd_ollama
resource: src/cli/mod.rs#L2470-L2561
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/cli/ollama_host
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/list_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/show_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/generate_embeddings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_ollama(config: &crate::config::Config, operation: OllamaCommands) -> Result<()>`

# Calls

- [ollama_host](../../../functions/src/cli/ollama_host.md)
- [list_models](../../../functions/src/llm/provider/ollama_models/list_models.md)
- [pull_model](../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [context](../../../functions/src/services/ServiceManager/context.md)
- [with_context](../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [show_model](../../../functions/src/llm/provider/ollama_models/show_model.md)
- [generate_embeddings](../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [next](../../../functions/src/tui/events/EventHandler/next.md)
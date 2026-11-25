---
type: Rust Method
title: with_context
resource: src/tui/error.rs#L149-L152
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/from_env
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/from_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/save_to_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/secrets/SecretString/delete_from_keyring
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_named_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/delete_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/show_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/delete_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/pull_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/generate_embeddings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/send_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/read_response_line
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_context(mut self, context: String) -> Self`

# Called by

- [configure_agent](../../../../../functions/src/cli/configure_agent.md)
- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [from_layers](../../../../../functions/src/config/Config/from_layers.md)
- [save](../../../../../functions/src/config/Config/save.md)
- [from_env](../../../../../functions/src/config/secrets/SecretString/from_env.md)
- [from_keyring](../../../../../functions/src/config/secrets/SecretString/from_keyring.md)
- [save_to_keyring](../../../../../functions/src/config/secrets/SecretString/save_to_keyring.md)
- [delete_from_keyring](../../../../../functions/src/config/secrets/SecretString/delete_from_keyring.md)
- [connect](../../../../../functions/src/db/Database/connect.md)
- [create_named_provider](../../../../../functions/src/llm/provider/factory/create_named_provider.md)
- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [delete_model](../../../../../functions/src/llm/provider/llama_cpp_models/delete_model.md)
- [show_model](../../../../../functions/src/llm/provider/ollama_models/show_model.md)
- [delete_model](../../../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [generate_embeddings](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [connect](../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)
- [read_response_line](../../../../../functions/src/mcp/client/MCPClient/read_response_line.md)
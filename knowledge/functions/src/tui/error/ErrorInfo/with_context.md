---
type: Rust Method
title: with_context
resource: src/tui/error.rs#L149-L152
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/cli/cmd_keyring
  - functions/src/cli/cmd_ollama
  - functions/src/config/Config/merge_from_file
  - functions/src/config/Config/save
  - functions/src/config/secrets/SecretString/from_env
  - functions/src/config/secrets/SecretString/from_keyring
  - functions/src/config/secrets/SecretString/save_to_keyring
  - functions/src/config/secrets/SecretString/delete_from_keyring
  - functions/src/db/Database/connect
  - functions/src/llm/provider/ollama_models/client_for
  - functions/src/llm/provider/ollama_models/show_model
  - functions/src/llm/provider/ollama_models/delete_model
  - functions/src/llm/provider/ollama_models/pull_model
  - functions/src/llm/provider/ollama_models/generate_embeddings
  - functions/src/mcp/client/MCPClient/connect
  - functions/src/mcp/client/MCPClient/send_request
  - functions/src/mcp/client/MCPClient/read_response_line
---

# Signature

`pub fn with_context(mut self, context: String) -> Self`

# Called by

- [cmd_keyring](../../../../../functions/src/cli/cmd_keyring.md)
- [cmd_ollama](../../../../../functions/src/cli/cmd_ollama.md)
- [merge_from_file](../../../../../functions/src/config/Config/merge_from_file.md)
- [save](../../../../../functions/src/config/Config/save.md)
- [from_env](../../../../../functions/src/config/secrets/SecretString/from_env.md)
- [from_keyring](../../../../../functions/src/config/secrets/SecretString/from_keyring.md)
- [save_to_keyring](../../../../../functions/src/config/secrets/SecretString/save_to_keyring.md)
- [delete_from_keyring](../../../../../functions/src/config/secrets/SecretString/delete_from_keyring.md)
- [connect](../../../../../functions/src/db/Database/connect.md)
- [client_for](../../../../../functions/src/llm/provider/ollama_models/client_for.md)
- [show_model](../../../../../functions/src/llm/provider/ollama_models/show_model.md)
- [delete_model](../../../../../functions/src/llm/provider/ollama_models/delete_model.md)
- [pull_model](../../../../../functions/src/llm/provider/ollama_models/pull_model.md)
- [generate_embeddings](../../../../../functions/src/llm/provider/ollama_models/generate_embeddings.md)
- [connect](../../../../../functions/src/mcp/client/MCPClient/connect.md)
- [send_request](../../../../../functions/src/mcp/client/MCPClient/send_request.md)
- [read_response_line](../../../../../functions/src/mcp/client/MCPClient/read_response_line.md)